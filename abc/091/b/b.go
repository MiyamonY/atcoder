// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Dec 29 04:48:49 2018
//
package main

import (
	"fmt"
	"sort"
)

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func main() {
	var N, M int
	fmt.Scan(&N)

	arr := make([]string, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	fmt.Scan(&M)
	arr1 := make([]string, M)
	for i := range arr1 {
		fmt.Scan(&arr1[i])
	}

	ans := 0
	for i := range arr {
		m := 0
		for j := range arr {
			if arr[i] == arr[j] {
				m++
			}
		}
		for k := range arr1 {
			if arr[i] == arr1[k] {
				m--
			}
		}
		ans = max(ans, m)
	}

	fmt.Println(ans)
}
