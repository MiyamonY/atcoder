// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 15:28:19 2020
//
package main

import "fmt"

const MOD = 1e9 + 7

type pair struct {
	A int
	B int
}

var H, N int
var AB []pair

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	fmt.Scan(&H, &N)
	AB = make([]pair, N)
	for i := range AB {
		fmt.Scan(&AB[i].A, &AB[i].B)
	}

	dp := make([]int, 1e4+1)
	for i := range dp {
		dp[i] = MOD
	}
	dp[0] = 0

	for _, ab := range AB {
		a := ab.A
		b := ab.B
		for i := range dp {
			if 0 <= i-a && dp[i-a] >= 0 {
				dp[i] = min(dp[i], dp[i-a]+b)
			}
		}
	}

	ans := int(MOD)
	for i := range dp {
		if i >= H {
			ans = min(ans, dp[i])
		}
	}
	fmt.Println(ans)
}
