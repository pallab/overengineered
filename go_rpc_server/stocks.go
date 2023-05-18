package main

type Market struct {
	tickers []string
}

func NewMarket() Market {
	s := Market{}
	r := Rand{}
	for i := 0; i < 10; i++ {
		s.tickers = append(s.tickers, r.RandString(3))
	}
	return s
}
