// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 21:11:18 2018
//
package main

import "fmt"

func main() {
	var N, X int
	fmt.Scan(&N, &X)

	total := 0
	min := 1 << 50
	for i := 0; i < N; i++ {
		var m int
		fmt.Scan(&m)
		total += m
		if m < min {
			min = m
		}
	}
	fmt.Println(N + (X-total)/min)
}
