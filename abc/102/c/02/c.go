// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 24 11:38:31 2019
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

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)
	slice := make([]int, N)
	scanSlice(slice)

	for i := range slice {
		slice[i] -= i + 1
	}
	sort.Ints(slice)

	mid := slice[len(slice)/2]

	ans := 0
	sum := 0
	for _, s := range slice {
		sum += abs(s - mid)
	}

	ans = sum
	if len(slice) > 1 {
		mid = slice[len(slice)/2-1]
		sum = 0
		for _, s := range slice {
			sum += abs(s - mid)
		}
		if sum < ans {
			ans = sum
		}
	}

	fmt.Println(ans)
}
