// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 17:22:37 2019
//
package main

import "fmt"

func main() {
	var A, B, C, D int
	fmt.Scan(&A, &B, &C, &D)

	if A+B == C+D {
		fmt.Println("Balanced")
	} else if A+B > C+D {
		fmt.Println("Left")
	} else {
		fmt.Println("Right")
	}
}
