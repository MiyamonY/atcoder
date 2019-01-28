// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Jan 28 11:49:52 2019
//
package main

import (
	"fmt"
	"sort"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var n int
	fmt.Scan(&n)

	slice := make([]int, n)
	for i := range slice {
		fmt.Scan(&slice[i])
	}
	sort.Ints(slice)

	a := slice[len(slice)-1]
	b := 0
	diff := 1 << 60
	for i := range slice {
		d := abs(a - 2*slice[i])
		if d < diff {
			diff = d
			b = slice[i]
		}
	}
	fmt.Println(a, b)
}
