// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 17:43:02 2018
//
package main

import "fmt"

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	arr := make([]int, N+1)
	for i := 1; i < len(arr); i++ {
		fmt.Scan(&arr[i])
	}

	var ans int64
	counts := map[int]int64{}
	for i := range arr {
		if i+1 < len(arr) {
			arr[i+1] += arr[i]
			arr[i+1] %= M
		}
		ans += counts[arr[i]]
		counts[arr[i]]++
	}
	fmt.Println(ans)
}
