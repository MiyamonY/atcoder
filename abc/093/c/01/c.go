// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 15:48:15 2018
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	arr := make([]int, 3)
	for i := range arr {
		fmt.Scan(&arr[i])
	}
	sort.Ints(arr)

	ans := arr[2] - arr[1]
	diff := arr[2] - (arr[0] + ans)
	if diff%2 == 0 {
		fmt.Println(ans + diff/2)
	} else {
		fmt.Println(ans + (diff+1)/2 + 1)
	}
}
