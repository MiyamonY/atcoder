// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Dec 26 21:24:13 2018
//
package main

import "fmt"

func abs(a int) int {
	if a <= 0 {
		return -a
	}
	return a
}

func main() {
	var a, b, c, d int
	fmt.Scan(&a, &b, &c, &d)
	if (abs(a-b) <= d && abs(b-c) <= d) || abs(a-c) <= d {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
