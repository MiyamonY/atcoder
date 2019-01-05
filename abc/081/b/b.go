// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 15:33:40 2019
//
package main

import (
	"fmt"
	"sort"
)

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)

		count := 0
		for a%2 == 0 {
			a /= 2
			count++
		}
		arr[i] = count
	}
	fmt.Println(min(arr...))
}
