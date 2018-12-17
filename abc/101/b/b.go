// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sun Dec 16 19:34:40 2018
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var N string
	fmt.Scan(&N)

	divider := 0
	for _, c := range N {
		divider += int(c - '0')
	}

	n, _ := strconv.Atoi(N)

	if n%divider == 0 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
