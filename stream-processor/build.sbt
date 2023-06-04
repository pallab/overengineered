ThisBuild / version := "0.1.0"

ThisBuild / scalaVersion := "2.12.17"

lazy val root = (project in file("."))
  .settings(
    name := "stream-processor",
    assembly / assemblyJarName := "streamer.jar",
    idePackagePrefix := Some("overengineered"),
  )

ThisBuild / assemblyMergeStrategy := {
  case PathList("org", "apache", xs @ _*)         => MergeStrategy.first
  case PathList(ps @ _*) if ps.last endsWith "module-info.class" => MergeStrategy.discard
  case PathList(ps @ _*) if ps.last endsWith ".properties" => MergeStrategy.first
  case PathList(ps @ _*) if ps.last endsWith ".proto" => MergeStrategy.first  
  case PathList(ps @ _*) if ps.last endsWith ".dat" => MergeStrategy.first  
  case "application.conf"                            => MergeStrategy.concat
  case "unwanted.txt"                                => MergeStrategy.discard
  case x =>
    val oldStrategy = (ThisBuild / assemblyMergeStrategy).value
    oldStrategy(x)
}


libraryDependencies += "org.apache.spark" %% "spark-core" % "3.4.0"
libraryDependencies += "org.apache.spark" %% "spark-sql" % "3.4.0"
libraryDependencies += "org.apache.spark" %% "spark-sql-kafka-0-10" % "3.4.0"