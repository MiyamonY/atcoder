// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 17:10:15 2019
//
package main

import (
	"fmt"
	"sort"
)

func scanSlice(slice []int) {
	for i := range slice {
		fmt.Scan(&slice[i])
	}
}

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	slice := make([]int, N)
	scanSlice(slice)

	ans := 0
	for i := range slice {
		ans += min(slice[i]*2, abs(slice[i]-K)*2)
	}

	fmt.Println(ans)
}
