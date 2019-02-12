// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:05:02 2019
//
package main

import "fmt"

var dp []int

func lucas(n int) int {
	if dp[n] == 0 {
		dp[n] = lucas(n-1) + lucas(n-2)
	}

	return dp[n]
}

func main() {
	var N int
	fmt.Scan(&N)

	dp = make([]int, 87)
	dp[0] = 2
	dp[1] = 1

	fmt.Println(lucas(N))
}
