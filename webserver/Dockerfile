# we will use multi-stage build feature to keep the final image size small
FROM rust:1.67 as build

WORKDIR /app

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

#install npm to build ui files
RUN apt-get -y install curl gnupg
RUN curl -sL https://deb.nodesource.com/setup_20.x  | bash -
RUN apt-get -y install cmake
RUN apt-get -y install nodejs
RUN apt-get -y install libmariadb3
RUN apt-get install -y librdkafka-dev
#RUN apt-get install libmariadb-dev

# Create a dummy project to pull the dependencies
RUN USER=root cargo new webserver

WORKDIR /app/webserver

COPY ./Cargo.toml ./
RUN cargo build --release

COPY ./src ./src
#COPY config.docker.json ./
RUN pwd && ls && ls -alth src
RUN rm -r target
RUN cargo build --release
RUN dpkg --print-architecture

# pull the react dependencies as well
WORKDIR /app/webserver/ui

COPY ui/package.json ./
RUN npm install

# build and export the ui as static files
COPY ./ui ./
RUN npm run build

# build the rust app
WORKDIR /app/webserver
RUN pwd && ls -alth ui/out && ls -alth src

# create the release build
FROM debian:bullseye
RUN dpkg --print-architecture
RUN apt-get update
RUN apt-get -y install libmariadb3
RUN apt-get install -y librdkafka-dev
#RUN apt-get install libmariadb-dev


EXPOSE 8080
WORKDIR /app/webserver

COPY --from=build /app/webserver/target/release/webserver ./
COPY --from=build /app/webserver/ui/out ./ui/out
RUN pwd && ls -alth && ls ui/out

ENV RUST_BACKTRACE=full
CMD [ "./webserver" ]
