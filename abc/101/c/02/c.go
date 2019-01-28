// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 24 15:48:11 2019
//
package main

import "fmt"

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	fmt.Println((N - 1 + K - 2) / (K - 1))
}
