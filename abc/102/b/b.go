// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Dec 10 23:55:28 2018
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
