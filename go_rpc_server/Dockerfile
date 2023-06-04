# we will use multi-stage build feature to keep the final image size small
FROM golang:1.20.4-buster as build

WORKDIR /usr/src/app

COPY go.mod go.sum ./
RUN pwd && ls -alth

RUN go mod download && go mod verify 
RUN pwd && ls -alth

COPY . . 
RUN pwd && ls -alth

RUN go build -v 
RUN pwd  && ls -alth

# create the release build
FROM debian:buster

EXPOSE 8089
WORKDIR /go_rpc_server

COPY --from=build /usr/src/app/rpc ./
RUN pwd && ls -alth

#ENV CONFIG_FILE=config.docker.json
CMD [ "./rpc" ]
