package main

import "time"

type Market struct {
	tickers map[string]time.Time
}

func NewMarket() Market {
	m := Market{tickers: make(map[string]time.Time, 10)}
	r := Rand{}

	for i := 0; i < 10; i++ {
		k := r.RandString(3)
		m.tickers[k] = time.Now().Add(time.Duration(-48) * time.Hour)
	}

	return m
}
