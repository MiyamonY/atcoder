// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Dec 23 23:26:20 2018
//
package main

import "fmt"

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func main() {
	var N int
	fmt.Scan(&N)
	if N == 0 {
		fmt.Println(0)
		return
	}

	ans := make([]int, 0)
	for N != 0 {
		var rem int
		if N < 0 && N%-2 != 0 {
			N--
			rem = 1
		}
		rem += abs(N % 2)
		ans = append(ans, rem)
		N /= -2
	}

	for i := len(ans) - 1; i >= 0; i-- {
		fmt.Print(ans[i])
	}
	fmt.Println()
}
