// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan 16 16:34:18 2019
//
package main

import "fmt"

const mod = 1e9 + 7

var dp = map[int]int{}

func fact(n int) int {
	if v, ok := dp[n]; ok {
		return v
	}
	dp[n] = (n * fact(n-1)) % mod
	return dp[n]
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	dp[0] = 1
	if N == M {
		fmt.Println((2 * (fact(N) * fact(M)) % mod) % mod)
	} else if N-1 == M || N == M-1 {
		fmt.Println((fact(N) * fact(M)) % mod)
	} else {
		fmt.Println(0)
	}
}
