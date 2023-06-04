# we will use multi-stage build feature to keep the final image size small
FROM sbtscala/scala-sbt:graalvm-ce-21.2.0-java8_1.8.1_2.12.17 as build

# its not possible to build the jar here becuase of 
# https://stackoverflow.com/questions/72842770/sbt-1-6-2-java-io-ioexception-function-not-implemented
# hence copying a pre-built jar
COPY ./target/scala-2.12/streamer.jar .

RUN pwd && ls -alth

CMD ["java", "-jar", "streamer.jar"]
