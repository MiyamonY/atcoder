// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Mon Dec 10 23:54:24 2018
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)
	if N%2 == 0 {
		fmt.Println(N)
		return
	}
	fmt.Println(2 * N)
}
