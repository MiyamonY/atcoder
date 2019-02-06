// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 17:27:56 2019
//
package main

import "fmt"

func pow2(n int) int {
	if n == 0 {
		return 1
	}
	return 2 * pow2(n-1)
}

func main() {
	var X, Y int
	fmt.Scan(&X, &Y)

	i := 0
	for ; pow2(i)*X <= Y; i++ {
	}
	fmt.Println(i)
}
