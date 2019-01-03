// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 22:45:53 2019
//
package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	if a*b%2 == 0 {
		fmt.Println("Even")
	} else {
		fmt.Println("Odd")
	}
}
