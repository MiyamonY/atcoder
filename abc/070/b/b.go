// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 20:56:28 2019
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

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func main() {
	var A, B, C, D int
	fmt.Scan(&A, &B, &C, &D)
	fmt.Println(max(0, min(B, D)-max(A, C)))
}
