// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 00:30:57 2019
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
	var X, t int
	fmt.Scan(&X, &t)

	fmt.Println(max(X-t, 0))
}
