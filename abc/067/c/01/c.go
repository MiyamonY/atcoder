// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:32:31 2019
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

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func main() {
	var N int
	fmt.Scan(&N)
	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}
	for i := range arr {
		if i+1 < len(arr) {
			arr[i+1] += arr[i]
		}
	}

	ans := 1 << 60
	for i := 0; i < len(arr)-1; i++ {
		ans = min(ans, abs(arr[i]-(arr[len(arr)-1]-arr[i])))
	}

	fmt.Println(ans)
}
