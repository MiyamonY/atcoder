// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:31:09 2019
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}
	sort.Ints(arr)

	ans := 0
	for i := 0; i < K; i++ {
		ans += arr[len(arr)-1-i]
	}
	fmt.Println(ans)
}
