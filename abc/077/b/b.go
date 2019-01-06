// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sun Jan  6 00:23:11 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	var i int
	for ; i*i <= N; i++ {
	}
	fmt.Println((i - 1) * (i - 1))
}
