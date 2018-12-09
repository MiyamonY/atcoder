// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 20:05:49 2018
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
	for i := 0; i < len(S); i++ {
		switch S[i] {
		case 'A':
			dp[i+1][0] = dp[i][0]
			dp[i+1][1] = (dp[i][1] + dp[i][0]) % MOD
			dp[i+1][2] = dp[i][2]
			dp[i+1][3] = dp[i][3]
		case 'B':
			dp[i+1][0] = dp[i][0]
			dp[i+1][1] = dp[i][1]
			dp[i+1][2] = (dp[i][2] + dp[i][1]) % MOD
			dp[i+1][3] = dp[i][3]
		case 'C':
			dp[i+1][0] = dp[i][0]
			dp[i+1][1] = dp[i][1]
			dp[i+1][2] = dp[i][2]
			dp[i+1][3] = (dp[i][3] + dp[i][2]) % MOD
		case '?':
			dp[i+1][0] = 3 * dp[i][0] % MOD
			dp[i+1][1] = (3*dp[i][1] + dp[i][0]) % MOD
			dp[i+1][2] = (3*dp[i][2] + dp[i][1]) % MOD
			dp[i+1][3] = (3*dp[i][3] + dp[i][2]) % MOD
		}
	}

	fmt.Println(dp[len(S)][3])
}
