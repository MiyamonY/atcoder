// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 15:16:32 2018
//
package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)

	arr := make([]int, n)
	for i := range arr {
		fmt.Scan(&arr[i])
	}
	sort.Ints(arr)

	a, b := arr[len(arr)-1], 0
	mid := float64(a) / 2.0
	dist := math.Abs(mid - float64(arr[0]))
	for i := range arr {
		d := math.Abs(mid - float64(arr[i]))
		if d < dist {
			dist = d
			b = arr[i]
		}
	}
	fmt.Println(a, b)
}
