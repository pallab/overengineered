package overengineered

import org.apache.log4j.Logger
import org.apache.log4j.Level
import org.apache.spark.sql.SparkSession
import org.apache.spark.sql.functions._
import org.apache.spark.sql.streaming.Trigger
import org.apache.spark.sql.types.DataTypes

import java.util.UUID

object Main {
  def main(args: Array[String]): Unit = {
    val kafka = scala.sys.env.getOrElse("KAFKA_SERVER", "localhost:9092")
    val topicSource = scala.sys.env.getOrElse("TOPIC_SOURCE", "words")
    val topicSink = scala.sys.env.getOrElse("TOPIC_SINK", "letter_counts")

    println(s"Starting Streamer. \nkafka : $kafka \ntopic : $topicSource -> $topicSink")

    Logger.getLogger("org").setLevel(Level.ERROR)

    val checkpointAt = "/tmp/spark-checkpoint" + UUID.randomUUID()

    val spark = SparkSession.builder()
      .appName("LetterCounter")
      .config("spark.master", "local")
      .config("spark.cores.max", 2)
      .config("spark.default.parallelism", 1)
      .config("spark.sql.streaming.statefulOperator.checkCorrectness.enabled", false)
      .getOrCreate()

    spark.sparkContext.setLogLevel("ERROR")

    import spark.implicits._

    val words = spark.readStream.format("kafka")
      .option("kafka.bootstrap.servers", kafka)
      .option("subscribe", topicSource)
      .option("startingOffsets", "earliest")
      .option("failOnDataLoss", false)
      .load()
      .selectExpr("cast(value as string)")
      .as[String]

    val letterCounts = words
      .flatMap(_.toCharArray.map(_.toString))
      .groupBy("value")
      .count()
      .withColumnRenamed("value", "c")
      .toJSON
      .withColumn("key", lit(1))
      .groupBy("key")
      .agg(collect_list("value").cast(DataTypes.StringType).as("value"))
      .withColumn("key", lit(java.time.LocalDateTime.now().toString))

    //        val query = letterCounts.writeStream.outputMode("complete")
    //          .format("console")
    //          .option("checkpointLocation", checkpointAt)
    //          .option("truncate", false)
    //          .start()

    val query = letterCounts
      .writeStream
      .outputMode("complete")
      .format("kafka")
      .trigger(Trigger.ProcessingTime("2 seconds"))
      .option("kafka.bootstrap.servers", kafka)
      .option("topic", topicSink)
      .option("checkpointLocation", checkpointAt)
      .start()

    query.awaitTermination()

  }
}