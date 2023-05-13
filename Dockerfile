# we will use multi-stage build feature to keep the final image size small
FROM rust:1.67 as build

WORKDIR /app

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

#install npm to build ui files
RUN apt-get -y install curl gnupg
RUN curl -sL https://deb.nodesource.com/setup_20.x  | bash -
RUN apt-get -y install nodejs

# Create a dummy project to pull the dependencies
RUN USER=root cargo new overengineered

WORKDIR /app/overengineered

COPY ./Cargo.toml ./
RUN cargo build --release

# pull the react dependencies as well
WORKDIR /app/overengineered/ui

COPY ./ui/package.json ./
RUN npm install

# build and export the ui as static files
COPY ./ui ./
RUN npm run build


# build the rust app
WORKDIR /app/overengineered
RUN pwd && ls static && ls -alth src

COPY ./src ./src
COPY ./config.json ./
RUN pwd && ls && ls -alth src

RUN cargo build --release

# create the release build
FROM debian:buster-slim

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

EXPOSE 8080
WORKDIR /app/overengineered

COPY --from=build /app/overengineered/target/release/overengineered ./
COPY --from=build /app/overengineered/config.json ./
COPY --from=build /app/overengineered/static ./static
RUN pwd && ls -alth && ls static

#ENV RUST_BACKTRACE=full
CMD [ "./overengineered" ]
