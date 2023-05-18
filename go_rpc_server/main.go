package main

import (
	"fmt"
	"google.golang.org/grpc"
	"log"
	"net"
	pb "overengineered.com/rpc/file_server"
)

func main() {
	fmt.Println("Hello Gophers !! ")

	listener, err := net.Listen("tcp", ":8089")

	if err != nil {
		log.Fatalf("can not create listener : %s", err)
	}

	server := grpc.NewServer()
	pb.RegisterFilesServer(server, &serverImpl{})

	server.Serve(listener)
}

type serverImpl struct {
	pb.FilesServer
}
