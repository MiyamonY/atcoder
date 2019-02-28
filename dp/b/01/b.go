// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 23:21:55 2019
//
package main

import "fmt"

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	dp := make([]int, N)
	for i := range arr {
		if i == 0 {
			continue
		}
		min := 1 << 60
		for j := 1; j <= K; j++ {
			if i-j >= 0 {
				n := dp[i-j] + abs(arr[i]-arr[i-j])
				if n < min {
					min = n
				}
			}
		}
		dp[i] = min
	}

	fmt.Println(dp[N-1])
}
