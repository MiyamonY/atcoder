// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 23:27:55 2019
//
package main

import (
	"fmt"
	"sort"
)

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func main() {
	var N int
	fmt.Scan(&N)

	a, b, c := make([]int, N), make([]int, N), make([]int, N)
	for i := 0; i < N; i++ {
		fmt.Scan(&a[i], &b[i], &c[i])
	}

	dp := make([][3]int, N+1)
	for i := 1; i <= N; i++ {
		dp[i][0] = a[i-1] + max(dp[i-1][1], dp[i-1][2])
		dp[i][1] = b[i-1] + max(dp[i-1][2], dp[i-1][0])
		dp[i][2] = c[i-1] + max(dp[i-1][0], dp[i-1][1])
	}

	fmt.Println(max(dp[N][0], dp[N][1], dp[N][2]))
}
