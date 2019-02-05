// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 22:05:19 2019
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
	var N int
	fmt.Scan(&N)
	arr1 := make([]int, N)
	arr2 := make([]int, N)

	for i := range arr1 {
		fmt.Scan(&arr1[i])
	}

	for i := range arr2 {
		fmt.Scan(&arr2[i])
	}

	for i := range arr1 {
		if i+1 < len(arr1) {
			arr1[i+1] += arr1[i]
			arr2[i+1] += arr2[i]
		}
	}

	ans := 0
	for i := range arr1 {
		l := 0
		if i > 0 {
			l = arr2[i-1]
		}
		ans = max(ans, arr1[i]+arr2[len(arr2)-1]-l)
	}

	fmt.Println(ans)
}
