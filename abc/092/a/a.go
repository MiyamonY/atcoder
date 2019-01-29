// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 23:08:46 2018
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
	var A, B, C, D int
	fmt.Scan(&A, &B, &C, &D)

	fmt.Println(min(A, B) + min(C, D))
}
