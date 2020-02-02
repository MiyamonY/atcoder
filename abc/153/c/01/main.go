// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 15:11:35 2020
//
package main

import (
	"fmt"
	"sort"
)

const MOD = 1e9 + 7

var N, K int
var H []int

func main() {
	fmt.Scan(&N, &K)
	H = make([]int, N)
	for i := range H {
		fmt.Scan(&H[i])
	}
	sort.Ints(H)

	ans := 0
	for i := 0; i < N-K; i++ {
		ans += H[i]
	}
	fmt.Println(ans)
}
