// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 16:15:34 2018
//
package main

import "fmt"

func main() {
	var A, B int
	fmt.Scan(&A, &B)

	if A <= 8 && B <= 8 {
		fmt.Println("Yay!")
	} else {
		fmt.Println(":(")
	}
}
