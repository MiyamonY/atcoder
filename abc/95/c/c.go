// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 21:13:28 2018
//
package main

import (
	"fmt"
	"sort"
)

func min(args ...int) int {
	sort.Ints(args)
	return args[0]
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var A, B, C, X, Y int
	fmt.Scan(&A, &B, &C, &X, &Y)

	withoutHalf := A*X + B*Y
	withHalfMin := 2*C*min(X, Y) + A*(X-min(X, Y)) + B*(Y-min(X, Y))
	withHalfMax := 2 * C * max(X, Y)

	fmt.Println(min(withoutHalf, withHalfMin, withHalfMax))
}
