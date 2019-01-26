// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 23:48:54 2018
//
package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	n := b - a

	fmt.Println(n*(n+1)/2 - b)
}
