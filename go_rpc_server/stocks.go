package main

import "time"

type Market struct {
	tickers map[string]time.Time
}

const numStocks = 10

func NewMarket() Market {
	m := Market{tickers: make(map[string]time.Time, 10)}
	r := Rand{}

	for i := 0; i < numStocks; i++ {
		k := r.RandString(3)
		m.tickers[k] = time.Now().Add(time.Duration(-48) * time.Hour)
	}

	return m
}
