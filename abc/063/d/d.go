// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 12:41:47 2019
//
package main

import (
	"fmt"
)

var N, A, B int

func canKillAll(monsters []int, n int) bool {
	count := 0
	for i := range monsters {
		rest := monsters[i] - n*B
		if rest > 0 {
			count += (rest + (A - B) - 1) / (A - B)
		}
	}
	return count <= n
}

func main() {
	fmt.Scan(&N, &A, &B)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	left := 0
	right := int(1e9)
	for left+1 < right {
		mid := (left + right) / 2
		if !canKillAll(arr, mid) {
			left = mid
		} else {
			right = mid
		}
	}
	fmt.Println(right)
}
