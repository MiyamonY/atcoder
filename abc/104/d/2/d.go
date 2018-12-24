// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 14:51:13 2018
//
package main

import "fmt"

const MOD = 1e9 + 7

func main() {
	var S string
	fmt.Scan(&S)

	dp := make([][]int, len(S)+1)
	for i := range dp {
		dp[i] = make([]int, 4)
	}

	dp[0][0] = 1
	for i, c := range S {
		if c == '?' {
			for j := range dp[i+1] {
				dp[i+1][j] += 3 * dp[i][j]
				dp[i+1][j] %= MOD
			}
		} else {
			for j := range dp[i+1] {
				dp[i+1][j] += dp[i][j]
				dp[i+1][j] %= MOD
			}
		}

		if c == 'A' || c == '?' {
			dp[i+1][1] += dp[i][0]
		}
		if c == 'B' || c == '?' {
			dp[i+1][2] += dp[i][1]
		}
		if c == 'C' || c == '?' {
			dp[i+1][3] += dp[i][2]
		}
		for j := range dp[i+1] {
			dp[i+1][j] %= MOD
		}
	}

	fmt.Println(dp[len(S)][3])
}
