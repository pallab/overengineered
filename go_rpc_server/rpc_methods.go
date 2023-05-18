package main

import (
	"context"
	"log"
	pb "overengineered.com/rpc/market"
	"time"
)

var market = NewMarket()
var random = Rand{}

func (s *serverImpl) ListStocks(context.Context, *pb.ListStocksRequest) (*pb.ListStocksResponse, error) {
	return &pb.ListStocksResponse{
		Names: market.tickers,
	}, nil
}

func (s *serverImpl) GetStockPrice(req *pb.StockPriceRequest, stream pb.StockMarket_GetStockPriceServer) error {
	log.Printf("received request : %v", req.Name)

	c := make(chan pb.StockPriceResponse, 100)

	go func(c chan pb.StockPriceResponse) {
		for i := 0; i < 100; i++ {
			for _, tick := range market.tickers {
				res := pb.StockPriceResponse{
					Timestamp: uint64(time.Now().Unix()),
					Ticker:    tick,
					Price:     uint32(random.RandNum(32, 67)),
					Volume:    uint32(random.RandNum(45, 56)),
				}
				c <- res
			}
			time.Sleep(1 * time.Second)
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
