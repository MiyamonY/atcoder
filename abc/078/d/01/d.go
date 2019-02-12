// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 18:05:57 2019
//
package main

import (
	"fmt"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func max(slice ...int) int {
	ret := slice[0]
	for _, v := range slice {
		if ret < v {
			ret = v
		}
	}
	return ret
}

func min(slice ...int) int {
	ret := slice[0]
	for _, v := range slice {
		if v < ret {
			ret = v
		}
	}
	return ret
}

func main() {
	var N, Z, W int
	fmt.Scan(&N, &Z, &W)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	if N == 1 {
		fmt.Println(abs(arr[0] - W))
	} else {
		last := arr[len(arr)-1]
		lastPred := arr[len(arr)-2]
		fmt.Println(max(abs(W-last), abs(last-lastPred)))
	}
}
