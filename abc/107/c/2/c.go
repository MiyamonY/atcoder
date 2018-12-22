// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec 22 02:29:35 2018
//
package main

import (
	"fmt"
	"sort"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	left := make([]int, 0)
	right := make([]int, 0)
	for i := 0; i < N; i++ {
		var x int
		fmt.Scan(&x)
		if x < 0 {
			left = append(left, -x)
		} else {
			right = append(right, x)
		}
	}
	sort.Ints(left)
	sort.Ints(right)

	ans := 1 << 31
	for i := 0; i < len(left); i++ {
		if 0 <= K-i-2 && K-i-2 < len(right) {
			ans = min(ans, 2*left[i]+right[K-i-2])
		}
	}
	for i := 0; i < len(right); i++ {
		if 0 <= K-i-2 && K-i-2 < len(left) {
			ans = min(ans, 2*right[i]+left[K-i-2])
		}
	}

	if K <= len(left) {
		ans = min(ans, left[K-1])
	}
	if K <= len(right) {
		ans = min(ans, right[K-1])
	}

	fmt.Println(ans)
}
