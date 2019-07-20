// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jul 20 21:10:49 2019
//
package main

import "fmt"

func main() {
	var n, d int
	fmt.Scan(&n, &d)
	m := d*2 + 1
	fmt.Println((n + (m - 1)) / m)
}
