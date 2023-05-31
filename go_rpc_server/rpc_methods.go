package main

import (
	"log"
	"math/rand"
	pb "overengineered.com/rpc/words"
	"time"
)

func RandString(length int) string {
	b := make([]uint8, length, length)

	for i := range b {
		b[i] = uint8(rand.Intn(123-32) + 32)
	}

	return string(b)
}

func RandomWord() (res pb.GetWordsResponse, err error) {
	resp := pb.GetWordsResponse{
		Timestamp: uint64(time.Now().Unix()),
		Word:      RandString(rand.Intn(5) + 1),
	}
	return resp, nil
}

func (s *serverImpl) GetWords(req *pb.GetWordsRequest, stream pb.Words_GetWordsServer) error {
	log.Printf("received request : %v \n", req)

	c := make(chan pb.GetWordsResponse, 100)

	go func(c chan pb.GetWordsResponse) {

		count := 0

		for {
			p, err := RandomWord()
			if err != nil {
				log.Printf("Error : %v \n", err)
				break
			}
			count++
			c <- p
			time.Sleep(time.Duration(5 * time.Second))
		}

		close(c)
	}(c)

	for res := range c {
		if err := stream.Send(&res); err != nil {
			return err
		}
	}
	return nil
}
