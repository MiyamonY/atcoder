// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:31:25 2019
//
package main

import "fmt"

func pow2(n int) int {
	if n == 0 {
		return 1
	}
	return 2 * pow2(n-1)
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	fmt.Println((1900*M + 100*(N-M)) * pow2(M))
}
