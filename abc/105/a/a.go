// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 00:41:33 2018
//
package main

import "fmt"

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	if N%K == 0 {
		fmt.Println(0)
	} else {
		fmt.Println(1)
	}
}
