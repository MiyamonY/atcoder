// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Fri Dec 21 09:33:56 2018
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

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	var N, X int
	fmt.Scan(&N, &X)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
		arr[i] -= X
		arr[i] = abs(arr[i])
	}

	ans := arr[0]
	for _, a := range arr {
		ans = gcd(ans, a)
	}
	fmt.Println(ans)
}
