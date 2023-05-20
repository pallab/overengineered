package main

import (
	"context"
	"errors"
	"log"
	pb "overengineered.com/rpc/market"
	"time"
)

var market = NewMarket()
var random = Rand{}

func (s *serverImpl) ListStocks(context.Context, *pb.ListStocksRequest) (*pb.ListStocksResponse, error) {

	var names []string

	for k, _ := range market.tickers {
		names = append(names, k)
	}

	return &pb.ListStocksResponse{
		Names: names,
	}, nil
}

func randomStockPrice() (res pb.StockPriceResponse, err error) {

	var availableKeys []string

	for k, v := range market.tickers {
		if time.Now().Sub(v).Minutes() > 1 {
			availableKeys = append(availableKeys, k)
		}
	}

	if len(availableKeys) > 0 {
		randomIdx := random.RandNum(0, len(availableKeys))
		randomKey := availableKeys[randomIdx]
		lastUpdatedAt := market.tickers[randomKey]

		newUpdatedTime := lastUpdatedAt.Add(time.Duration(1 * time.Minute))
		resp := pb.StockPriceResponse{
			Timestamp: uint64(newUpdatedTime.Unix()),
			Ticker:    randomKey,
			Price:     uint32(random.RandNum(32, 67)),
			Volume:    uint32(random.RandNum(45, 56)),
		}
		market.tickers[randomKey] = newUpdatedTime
		log.Printf("sending res :%T, %v \n", resp, resp)
		return resp, nil
	}
	return pb.StockPriceResponse{}, errors.New("no more ")
}

func (s *serverImpl) GetStockPrice(req *pb.StockPriceRequest, stream pb.StockMarket_GetStockPriceServer) error {
	log.Printf("received request : %v \n", req.Name)

	for {
		res, err := randomStockPrice()
		if err != nil {
			log.Printf("Error : %v \n", err)
			break
		} else if err := stream.Send(&res); err != nil {
			return err
		}
	}

	return nil
}

func (s *serverImpl) GetStockPrice_(req *pb.StockPriceRequest, stream pb.StockMarket_GetStockPriceServer) error {
	log.Printf("received request : %v \n", req.Name)

	c := make(chan pb.StockPriceResponse, 100)

	go func(c chan pb.StockPriceResponse) {

		for {
			p, err := randomStockPrice()
			if err != nil {
				log.Printf("Error : %v \n", err)
				break
			}
			c <- p
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
