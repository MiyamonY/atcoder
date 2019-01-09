// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 09:18:51 2019
//
package main

import "fmt"

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var x, a, b int
	fmt.Scan(&x, &a, &b)
	if abs(x-a) < abs(x-b) {
		fmt.Println("A")
	} else {
		fmt.Println("B")
	}
}
