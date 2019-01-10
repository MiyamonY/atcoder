// Package main provides
//
// File:  e.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 21:23:06 2019
//
package main

import (
	"fmt"
	"sort"
)

type item struct {
	w, v int
}

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func main() {
	var N, W int
	fmt.Scan(&N, &W)
	items := make([]item, N)

	for i := range items {
		fmt.Scan(&items[i].w, &items[i].v)
	}

	dp := make([][]int, N+1)
	for i := range dp {
		dp[i] = make([]int, 1e5+1)
		for j := range dp[i] {
			dp[i][j] = 1 << 60
		}
	}
	dp[0][0] = 0

	for i := range items {
		w, v := items[i].w, items[i].v
		for j := range dp[i+1] {
			if j-v >= 0 {
				dp[i+1][j] = min(dp[i][j], dp[i][j-v]+w)
			} else {
				dp[i+1][j] = dp[i][j]
			}
		}
	}
	ans := 0

	for i := range dp[N] {
		if dp[N][i] <= W {
			ans = i
		}
	}
	fmt.Println(ans)
}
