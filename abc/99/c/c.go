// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Tue Dec 25 00:43:02 2018
//
package main

import (
	"fmt"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	var N int
	fmt.Scan(&N)
	dp := make([]int, 1e5+1)

	for i := range dp {
		dp[i] = i
	}

	for i := range dp {
		for j := 6; j <= 1e5+0; j *= 6 {
			if i+j <= 1e5 {
				dp[i+j] = min(dp[i+j], dp[i]+1)
			}
		}

		for j := 9; j <= 1e5+0; j *= 9 {
			if i+j <= 1e5 {
				dp[i+j] = min(dp[i+j], dp[i]+1)
			}
		}
	}

	fmt.Println(dp[N])
}
