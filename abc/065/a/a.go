// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Jan 16 16:19:53 2019
//
package main

import "fmt"

func main() {
	var X, A, B int
	fmt.Scan(&X, &A, &B)

	if B-A <= 0 {
		fmt.Println("delicious")
	} else if B-A <= X {
		fmt.Println("safe")
	} else {
		fmt.Println("dangerous")
	}
}
