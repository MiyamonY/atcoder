// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 18:38:17 2018
//
package main

import "fmt"

func main() {
	var R int
	fmt.Scan(&R)

	if R < 1200 {
		fmt.Println("ABC")
	} else if R < 2800 {
		fmt.Println("ARC")
	} else {
		fmt.Println("AGC")
	}
}
