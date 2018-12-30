// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 21:25:38 2018
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
	var N, K int
	_, _ = fmt.Scan(&N, &K)

	ans := 0
	for b := K + 1; b <= N; b++ {
		ans += max(0, N/b*(b-K)) + max(0, (N%b)-K+1)
	}

	if K == 0 {
		ans -= N - K
	}

	fmt.Println(ans)
}
