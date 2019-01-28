// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 14:58:13 2018
//
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	var N, M, X int
	fmt.Scan(&N, &M, &X)

	arr := make([]int, N+1)
	for i := 0; i < M; i++ {
		var a int
		fmt.Scan(&a)
		arr[a]++
	}

	tmp1 := 0
	for i := X; i < len(arr); i++ {
		tmp1 += arr[i]
	}

	tmp2 := 0
	for i := X; i >= 0; i-- {
		tmp2 += arr[i]
	}

	fmt.Println(min(tmp1, tmp2))
}
