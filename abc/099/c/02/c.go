// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Jan 26 13:33:14 2019
//
package main

import (
	"fmt"
)

func pow(n, m int) int {
	if m == 0 {
		return 1
	}
	return n * pow(n, m-1)
}

func min(slice ...int) int {
	ret := slice[0]
	for _, s := range slice {
		if s < ret {
			ret = s
		}
	}
	return ret
}

func main() {
	var N int
	fmt.Scan(&N)

	dp := make([]int, N+1)
	for i := range dp {
		dp[i] = i
	}

	for i := range dp {
		for j := 6; i+j <= N; j *= 6 {
			dp[i+j] = min(dp[i+j], dp[i]+1)
		}
		for j := 9; i+j <= N; j *= 9 {
			dp[i+j] = min(dp[i+j], dp[i]+1)
		}
	}

	fmt.Println(dp[N])
}
