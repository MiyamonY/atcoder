// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 11:42:47 2019
//
package main

import "fmt"

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	dp := make([]bool, N*100)
	dp[0] = true

	for i := range arr {
		for j := len(dp) - 1; j >= 0; j-- {
			if 0 <= j-arr[i] && dp[j-arr[i]] {
				dp[j] = true
			}
		}
	}

	ans := 0
	for j := range dp {
		if dp[j] && j%10 != 0 {
			ans = max(ans, j)
		}
	}

	fmt.Println(ans)
}
