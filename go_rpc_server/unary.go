package main

import (
	"context"
	pb "overengineered.com/rpc/messenger"
)

func (s *serverImpl) ListFiles(context.Context, *pb.ListFilesRequest) (*pb.ListFilesResponse, error) {
	return &pb.ListFilesResponse{
		Names: []string{"a", "b"},
	}, nil
}
