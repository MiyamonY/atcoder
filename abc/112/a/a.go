// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 12:46:21 2018
//
package main

import "fmt"

var n int

func main() {
	fmt.Scan(&n)
	if n == 1 {
		fmt.Println("Hello World")
	} else {
		var a, b int
		fmt.Scan(&a, &b)
		fmt.Println(a + b)
	}
}
