package main

import (
	"log"
	"net"
	"os"

	"google.golang.org/grpc"
	pb "overengineered.com/rpc/words"
)

func main() {
	log.Println("Starting go rpc server ")
	address := os.Getenv("BIND_ADDRESS")
	if address == "" {
		address = ":8089"
	}
	log.Printf("Bind address : %v\n", address)

	listener, err := net.Listen("tcp", address)

	if err != nil {
		log.Fatalf("can not create listener : %s", err)
	}

	server := grpc.NewServer()
	pb.RegisterWordsServer(server, &serverImpl{})

	server.Serve(listener)
}

type serverImpl struct {
	pb.WordsServer
}
