package main

import (
	"math/rand"
)

const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
const length = len(letters) - 1

type Rand struct{}

func (r *Rand) RandString(n int) string {
	b := make([]uint8, n, n)

	for i := range b {
		b[i] = letters[rand.Intn(length)]
	}

	return string(b)
}

func (r *Rand) RandNum(min int, max int) int {
	return rand.Intn(max-min) + min
}

func (r *Rand) RandNumNear(n int, deviationPercentage int) int {
	deviation := n * deviationPercentage / 100
	return r.RandNum(n-deviation, n+deviation)
}
