// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 17 14:30:43 2018
//
package main

import "fmt"

func nums(n int64) int64 {
	if n == 0 {
		return 1
	}
	return 3 + 2*nums(n-1)
}

func middle(n int64) int64 {
	return (nums(n) + 1) / 2
}

func pathis(n, x int64) int64 {
	if x == 0 {
		return 0
	}

	if n == 0 && x == 1 {
		return 1
	}

	mid := middle(n)
	if x < mid {
		return pathis(n-1, x-1)
	} else if x == nums(n) {
		return 1 + 2*pathis(n-1, nums(n-1))
	} else {
		return 1 + pathis(n-1, nums(n-1)) + pathis(n-1, x-mid)
	}
}

func main() {
	var N, X int64
	fmt.Scan(&N, &X)
	fmt.Println(pathis(N, X))
}
