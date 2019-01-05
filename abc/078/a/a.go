// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:26:13 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var X, Y string
	fmt.Scan(&X, &Y)
	x, _ := strconv.ParseUint(X, 16, 64)
	y, _ := strconv.ParseUint(Y, 16, 64)
	if x == y {
		fmt.Println("=")
	} else if x < y {
		fmt.Println("<")
	} else {
		fmt.Println(">")
	}
}
