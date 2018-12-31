// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Dec 31 13:18:10 2018
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

	isAlice := true
	alice := 0
	bob := 0
	for i := len(arr) - 1; i >= 0; i-- {
		if isAlice {
			alice += arr[i]
		} else {
			bob += arr[i]
		}
		isAlice = !isAlice
	}

	fmt.Println(alice - bob)
}
