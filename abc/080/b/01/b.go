// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 16:17:11 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var N int
	fmt.Scan(&N)

	n := strconv.Itoa(N)

	sum := 0
	for i := range n {
		sum += int(n[i] - '0')
	}

	if N%sum == 0 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
