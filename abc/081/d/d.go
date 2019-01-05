// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 15:54:59 2019
//
package main

import (
	"fmt"
)

func max(slice ...int) int {
	val := slice[0]
	for _, s := range slice {
		if val < s {
			val = s
		}
	}
	return val
}

func min(slice ...int) int {
	val := slice[0]

	for _, s := range slice {
		if s < val {
			val = s
		}
	}
	return val
}

func find(slice []int, val int) int {
	for i := range slice {
		if slice[i] == val {
			return i
		}
	}
	return len(slice)
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
	arr := make([]int, N)

	for i := range arr {
		fmt.Scan(&arr[i])
	}
	minVal := min(arr...)
	maxVal := max(arr...)

	fmt.Println(2*N - 1)
	if abs(minVal) < abs(maxVal) {
		index := find(arr, maxVal)

		for i := 0; i < N; i++ {
			fmt.Println(index+1, i+1)
		}

		for i := 1; i <= N; i++ {
			if i+1 <= N {
				fmt.Println(i, i+1)
			}
		}
	} else {
		index := find(arr, minVal)

		for i := 0; i < N; i++ {
			fmt.Println(index+1, i+1)
		}

		for i := N; i >= 1; i-- {
			if i-1 >= 1 {
				fmt.Println(i, i-1)
			}
		}
	}
}
