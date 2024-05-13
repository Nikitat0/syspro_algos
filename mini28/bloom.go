package main

import (
	"math"
	"math/big"
	"math/rand"
)

type Bloom struct {
	hashes []hashFunction
	bins   []uint64
}

func NewBloom(s uint, pr float64) Bloom {
	var bloom Bloom

	n := uint(float64(s) * math.Log(pr) / -math.Pow(math.Log(2), 2.0))
	nbins := n / 64
	if n%64 != 0 {
		nbins += 1
	}
	nbins = findPrime(nbins)
	nhashes := int(math.Ceil(float64(n) * math.Log(2) / float64(s)))
	if nhashes < 1 {
		nhashes = 1
	}

	bloom.bins = make([]uint64, nbins)
	bloom.hashes = make([]hashFunction, nhashes)
	for i := 0; i < nhashes; i++ {
		bloom.hashes[i].randomInit(int(nbins))
	}
	return bloom
}

func (b Bloom) Insert(ip Ip) {
	for _, f := range b.hashes {
		b.setBit(f.hash(ip) % b.size())
	}
}

func (b Bloom) Lookup(ip Ip) bool {
	for _, f := range b.hashes {
		if !b.getBit(f.hash(ip) % b.size()) {
			return false
		}
	}
	return true
}

func (b Bloom) size() int {
	return len(b.bins) * 64
}

func (b Bloom) nHashes() int {
	return len(b.hashes)
}

func (b Bloom) getBit(n int) bool {
	return *b.lookupWord(n)>>(n%64) != 0
}

func (b Bloom) setBit(n int) {
	*b.lookupWord(n) |= 1 << (n % 64)
}

func (b Bloom) lookupWord(n int) *uint64 {
	return &b.bins[n/64]
}

type hashFunction struct {
	a, b int
}

func (f *hashFunction) randomInit(m int) {
	f.a = rand.Intn(m)
	f.b = rand.Intn(m-1) + 1
}

func (f *hashFunction) hash(ip Ip) int {
	return f.a*int(ip.Repr()) + f.b
}

func findPrime(n uint) uint {
	for !big.NewInt(int64(n)).ProbablyPrime(0) {
		n++
	}
	return n
}
