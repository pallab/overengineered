package overengineered

import org.apache.log4j.Logger
import org.apache.log4j.Level
import org.apache.spark.sql.SparkSession
import org.apache.spark.sql.connector.catalog.Column
import org.apache.spark.sql.functions._
import org.apache.spark.sql.types.DataTypes

import java.util.UUID

object Main {
  case class CharCounts(c: String, count: Long)

  def main(args: Array[String]): Unit = {
    println("Hello world!")
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
      .option("kafka.bootstrap.servers", "localhost:9094")
      .option("subscribe", "words")
      .option("startingOffsets", "earliest")
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


    //    val query = c.writeStream.outputMode("complete")
    //      .format("console")
    //      .option("checkpointLocation", checkpointAt)
    //      .option("truncate", false)
    //      .start()

    val query = letterCounts
      .writeStream
      .outputMode("complete")
      .format("kafka")
      .option("kafka.bootstrap.servers", "localhost:9094")
      .option("topic", "letter_counts")
      .option("checkpointLocation", checkpointAt)
      .start()

    query.awaitTermination()

  }
}