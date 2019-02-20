// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Fri Jan 11 02:25:06 2019
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
	var a, b, c int
	fmt.Scan(&a, &b, &c)
	fmt.Println(min(a+b, b+c, c+a))
}
