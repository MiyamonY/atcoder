// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Jan 16 19:10:41 2019
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	sort.Ints(arr)
	fmt.Println(arr[len(arr)-1] - arr[0])
}
