// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 17 18:43:46 2018
//
package main

import "fmt"

const MOD = 1e9 + 7

func adjacent(net int) bool {
	for i := 0; i < 7; i++ {
		if net&(0x03<<uint8(i)) == 0x03<<uint8(i) {
			return true
		}
	}
	return false
}

func main() {
	var H, W, K int
	fmt.Scan(&H, &W, &K)
	dp := make([][]int, H+1)
	for i := range dp {
		dp[i] = make([]int, W)
	}
	dp[0][0] = 1

	for h := 1; h < H+1; h++ {
		for w := 0; w < len(dp[h]); w++ {
			for i := 0; i < 1<<uint8(W-1); i++ {
				if adjacent(i) {
					continue
				}
				if w+1 < W && i&(1<<uint8(w)) != 0 {
					dp[h][w] += dp[h-1][w+1] % MOD
				} else if w > 0 && i&(1<<uint8(w-1)) != 0 {
					dp[h][w] += dp[h-1][w-1] % MOD
				} else {
					dp[h][w] += dp[h-1][w] % MOD
				}
				dp[h][w] %= MOD
			}
		}
	}

	fmt.Println(dp[H][K-1])
}
