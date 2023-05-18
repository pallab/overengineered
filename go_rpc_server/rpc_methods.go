package main

import (
	"context"
	"log"
	pb "overengineered.com/rpc/file_server"
	"time"
)

func (s *serverImpl) ListFiles(context.Context, *pb.ListFilesRequest) (*pb.ListFilesResponse, error) {
	return &pb.ListFilesResponse{
		Names: []string{"a", "b"},
	}, nil
}

func (s *serverImpl) LoadFile(req *pb.LoadFileRequest, stream pb.Files_LoadFileServer) error {
	log.Printf("received request : %v", req.Name)

	for i := 0; i < 10; i++ {
		err := stream.Send(&pb.LoadFileResponse{
			IsSuccess: i%2 == 0,
		})
		if err != nil {
			return err
		}
		time.Sleep(2 * time.Second)
	}
	return nil
}
