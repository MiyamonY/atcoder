// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 02:20:14 2018
//
package main

import "fmt"

var x, y int

func main() {
	fmt.Scan(&x, &y)

	fmt.Println(x + y/2)
}
