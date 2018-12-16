// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Mon Dec 10 23:56:37 2018
//
package main

import (
	"fmt"
	"sort"
)

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
		arr[i] -= i + 1
	}
	b := make([]int, len(arr))
	copy(b, arr)

	sort.Ints(b)

	median := b[len(b)/2]
	ans := 0
	for i := range arr {
		ans += abs(arr[i] - median)
	}
	fmt.Println(ans)
}
