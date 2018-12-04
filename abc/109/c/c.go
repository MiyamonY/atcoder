// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Dec  5 00:37:04 2018
//
package main

import "fmt"

var n, x int

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func main() {
	fmt.Scan(&n, &x)
	arr := make([]int, n)

	for i := range arr {
		fmt.Scan(&arr[i])
		arr[i] = abs(arr[i] - x)
	}
	ans := arr[0]

	for i := range arr {
		ans = gcd(ans, arr[i])
	}
	fmt.Println(ans)
}
