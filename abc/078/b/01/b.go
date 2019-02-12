// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:29:43 2019
//
package main

import "fmt"

func main() {
	var X, Y, Z int
	fmt.Scan(&X, &Y, &Z)

	i := 0
	for ; Y*i+Z*(i+1) <= X; i++ {
	}
	fmt.Println(i - 1)
}
