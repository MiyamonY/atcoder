// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jan 18 17:39:17 2020
//
package main

import (
	"fmt"
	"sort"
)

const MOD = 1e9 + 7

type comb struct {
	memo []int
}

func (c *comb) fact(n int) int {
	if c.memo[n] > 0 {
		return c.memo[n]
	}
	c.memo[n] = n * c.fact(n-1) % MOD
	return c.memo[n]
}

func (c *comb) pow(n, k int) int {
	if k == 0 {
		return 1
	}
	if k%2 == 1 {
		return n * c.pow(n*n%MOD, k/2) % MOD
	}
	return c.pow(n*n%MOD, k/2)
}

func (c *comb) calc(n, k int) int {
	if n == 0 || k == 0 || n-k < 0 {
		return 0
	}

	return c.fact(n) * c.pow(c.fact(k), MOD-2) % MOD * c.pow(c.fact(n-k), MOD-2) % MOD
}

func newComb(n int) *comb {
	c := comb{}
	c.memo = make([]int, n+1)
	c.memo[0] = 1
	return &c
}

var N, K int
var A []int

func main() {
	fmt.Scan(&N, &K)
	A = make([]int, N)
	for i := range A {
		fmt.Scan(&A[i])
	}
	sort.Ints(A)

	c := newComb(N)
	ans := 0
	for i := 0; i < N; i++ {
		ans += c.calc(N-1-i, K-1) * A[N-1-i] % MOD
		ans -= c.calc(N-1-i, K-1) * A[i] % MOD
		ans += MOD
		ans %= MOD
	}
	fmt.Println(ans)
}
