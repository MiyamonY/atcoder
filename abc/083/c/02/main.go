// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Wed Feb  6 12:11:20 2019
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

	for i := 0; i < 1000; i++ {
		if X*pow2(i) <= Y && Y < X*pow2(i+1) {
			fmt.Println(i + 1)
			return
		}
	}
}
