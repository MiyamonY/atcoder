// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 15:05:23 2018
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

	tmp := make([]int, N)
	copy(tmp, arr)

	sort.Ints(tmp)

	left := tmp[len(arr)/2-1]
	right := tmp[len(arr)/2]

	for i := range arr {
		if arr[i] <= left {
			fmt.Println(right)
		} else {
			fmt.Println(left)
		}
	}
}
