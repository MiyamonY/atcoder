// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Fri Jan  4 15:54:17 2019
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

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)

	m := map[int]int{}
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		m[a]++
	}

	ans := 0
	for k, v := range m {
		if v >= k {
			ans += v - k
		} else {
			ans += v
		}
	}

	fmt.Println(ans)
}
