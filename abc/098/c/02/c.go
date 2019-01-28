// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 01:52:44 2019
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

	var S string
	fmt.Scan(&S)

	wests := make([]int, N)
	easts := make([]int, N)
	for i, s := range S {
		if s == 'W' {
			wests[i] = 1
		} else {
			easts[i] = 1
		}
	}
	for i := range wests {
		if i+1 < len(easts) {
			easts[i+1] += easts[i]
		}
		if N-1-i-1 >= 0 {
			wests[N-1-i-1] += wests[N-1-i]
		}
	}

	ans := 1 << 60
	for i := range easts {
		if i == 0 {
			ans = min(ans, N-1-wests[1])
		} else if i == N-1 {
			ans = min(ans, N-1-easts[N-2])
		} else {
			ans = min(ans, N-1-easts[i-1]-wests[i+1])
		}
	}
	fmt.Println(ans)
}
