// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 16:16:00 2019
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

func main() {
	var N, A, B int
	fmt.Scan(&N, &A, &B)

	fmt.Println(min(N*A, B))
}
