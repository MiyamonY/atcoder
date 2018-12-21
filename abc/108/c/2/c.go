// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec 22 00:53:43 2018
//
package main

import "fmt"

func main() {
	var N, K int
	fmt.Scan(&N, &K)
	ans := 0
	for a := 1; a <= K; a++ {
		b := K - a
		if b == 0 {
			b = K
		}
		if K-a != K-b {
			continue
		}
		c := K - b
		if c == 0 {
			c = K
		}
		if a > N || b > N || c > N {
			continue
		}
		ans += (1 + (N-a)/K) * (1 + (N-b)/K) * (1 + (N-c)/K)
	}

	fmt.Println(ans)
}
