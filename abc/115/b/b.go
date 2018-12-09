// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 21:13:28 2018
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}
	sort.Ints(arr)
	ans := 0
	for i := 0; i < N-1; i++ {
		ans += arr[i]
	}
	ans += arr[len(arr)-1] / 2

	fmt.Println(ans)
}
