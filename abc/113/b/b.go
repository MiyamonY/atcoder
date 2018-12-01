// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 02:24:03 2018
//
package main

import (
	"fmt"
	"math"
)

var n int
var t, a float64
var hs []int

func main() {
	fmt.Scan(&n)
	fmt.Scan(&t, &a)

	hs := make([]int, n)
	for i := range hs {
		fmt.Scan(&hs[i])
	}

	minIndex := 0
	minVal := 1e60
	for i := range hs {
		diff := math.Abs(a - (t - float64(hs[i])*0.006))
		if diff < minVal {
			minIndex = i
			minVal = diff
		}
	}

	fmt.Println(minIndex + 1)
}
