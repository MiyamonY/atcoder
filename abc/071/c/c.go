// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 09:22:05 2019
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
	for i := 0; i < N; i++ {
		fmt.Scan(&arr[i])
	}
	sort.Ints(arr)

	a, b := 0, 0
	index := 0
	for i := len(arr) - 1; i >= 0; i-- {
		if i > 0 && arr[i] == arr[i-1] {
			a = arr[i]
			index = i - 2
			break
		}
	}

	for i := index; i >= 0; i-- {
		if i > 0 && arr[i] == arr[i-1] {
			b = arr[i]
			break
		}
	}

	fmt.Println(a * b)
}
