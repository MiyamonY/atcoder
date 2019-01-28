// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Jan 28 13:05:39 2019
//
package main

import (
	"fmt"
	"sort"
)

// min ...
func min(args ...int) int {
	m := args[0]
	for i := range args {
		if args[i] < m {
			m = args[i]
		}
	}
	return m
}

func main() {
	arr := []int{0, 0, 0}
	fmt.Scan(&arr[0], &arr[1], &arr[2])
	sort.Ints(arr)

	diff1 := arr[2] - arr[1]
	diff2 := arr[2] - arr[0]
	if diff1%2 == 0 && diff2%2 == 0 {
		fmt.Println((diff1 + diff2) / 2)
	} else if diff1%2 == 0 || diff2%2 == 0 {
		fmt.Println((diff1+diff2+1)/2 + 1)
	} else {
		fmt.Println((diff1-1+diff2-1)/2 + 1)
	}
}
