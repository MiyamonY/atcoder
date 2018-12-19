// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 03:28:15 2018
//
package main

import "fmt"

const MOD = 1e9 + 7

var h, w, k int
var dp [][]int

func main() {
	fmt.Scan(&h, &w, &k)

	if w == 1 {
		if k == 1 {
			fmt.Println(1)
		} else {
			fmt.Println(0)
		}
		return
	}

	dp = make([][]int, h+1)
	for i := range dp {
		dp[i] = make([]int, w)
	}
	dp[0][0] = 1

	lines := make([]int, 0)

	for i := 0; i < 1<<uint64(w-1); i++ {
		invalid := false
		for j := 0; j < w; j++ {
			if i&(0x03<<uint64(j)) == (0x03 << uint64(j)) {
				invalid = true
			}
		}
		if invalid {
			continue
		}

		lines = append(lines, i)
	}

	for i := 1; i <= h; i++ {
		for j := 0; j < w; j++ {
			left, middle, right := 0, 0, 0
			for _, line := range lines {
				if line&(1<<uint64(j)) != 0 {
					right++
				} else if j > 0 && line&(1<<uint64(j-1)) != 0 {
					left++
				} else {
					middle++
				}
			}

			if j == 0 {
				dp[i][j] = right*dp[i-1][j+1]%MOD + middle*dp[i-1][j]%MOD
			} else if j == w-1 {
				dp[i][j] = middle*dp[i-1][j]%MOD + left*dp[i-1][j-1]%MOD
			} else {
				dp[i][j] = (left*dp[i-1][j-1]%MOD+middle*dp[i-1][j]%MOD)%MOD + right*dp[i-1][j+1]%MOD
			}
			dp[i][j] %= MOD
		}

	}
	fmt.Println(dp[h][k-1] % MOD)
}
