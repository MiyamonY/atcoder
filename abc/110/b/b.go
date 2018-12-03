// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sun Dec  2 04:51:26 2018
//
package main

import (
	"fmt"
	"sort"
)

var n, m, x, y int

func main() {
	fmt.Scan(&n, &m, &x, &y)

	xs := make([]int, n)
	ys := make([]int, m)

	for i := range xs {
		fmt.Scan(&xs[i])
	}
	sort.Ints(xs)
	max := xs[len(xs)-1]

	for i := range ys {
		fmt.Scan(&ys[i])
	}
	sort.Ints(ys)
	min := ys[0]

	sort.Ints(ys)
	for i := x + 1; i <= y; i++ {
		if max < i && i <= min {
			fmt.Println("No War")
			return
		}
	}
	fmt.Println("War")
}
