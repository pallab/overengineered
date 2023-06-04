package main

import (
	"log"
	"net"

	"google.golang.org/grpc"
	pb "overengineered.com/rpc/words"
)

func main() {
	log.Println("starting go rpc server ")
	listener, err := net.Listen("tcp", ":8089")

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
