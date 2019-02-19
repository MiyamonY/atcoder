// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 20:55:17 2019
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
	for i := range n {
		if n[i] != n[len(n)-1-i] {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
