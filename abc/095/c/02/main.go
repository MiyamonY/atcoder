// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 17:58:27 2019
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

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func main() {
	var A, B, C, X, Y int
	fmt.Scan(&A, &B, &C, &X, &Y)

	fmt.Println(min(A*X+B*Y, 2*C*max(X, Y), 2*C*min(X, Y)+A*(X-min(X, Y))+B*(Y-min(X, Y))))
}
