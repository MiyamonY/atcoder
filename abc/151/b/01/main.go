// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jan 18 17:03:48 2020
//
package main

import "fmt"

var N, M, K int
var A []int

func main() {
	fmt.Scan(&N, &K, &M)
	A = make([]int, N-1)
	sum := 0
	for i := range A {
		fmt.Scan(&A[i])
		sum += A[i]
	}

	diff := N*M - sum
	if diff <= 0 {
		fmt.Println(0)
	} else if diff <= K {
		fmt.Println(diff)
	} else {
		fmt.Println(-1)
	}
}
