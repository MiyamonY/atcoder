// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  3 21:01:00 2019
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N int
	fmt.Scan(&N)
	slice := make([]int, N)

	for i := range slice {
		fmt.Scan(&slice[i])
	}
	sort.Ints(slice)

	remTotal := 0
	for i := range slice {
		if i < len(slice)-1 {
			remTotal += slice[i]
		}
	}

	if slice[len(slice)-1] < remTotal {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
