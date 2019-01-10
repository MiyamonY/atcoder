// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 23:15:00 2019
//
package main

import (
	"fmt"
	"sort"
)

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func abs(a int) int {
	if a < 0 {
		return -a
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

	dp := make([]int, N)
	for i := range arr {
		if i > 1 {
			dp[i] = min(dp[i-1]+abs(arr[i]-arr[i-1]), dp[i-2]+abs(arr[i]-arr[i-2]))
		} else if i > 0 {
			dp[i] = dp[i-1] + abs(arr[i]-arr[i-1])
		}
	}

	fmt.Println(dp[N-1])
}
