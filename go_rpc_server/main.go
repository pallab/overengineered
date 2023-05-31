package main

import (
	"google.golang.org/grpc"
	"log"
	"net"
	pb "overengineered.com/rpc/words"
)

func main() {

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
