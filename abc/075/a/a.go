// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 01:51:47 2019
//
package main

import "fmt"

func main() {
	var A, B, C int
	fmt.Scan(&A, &B, &C)

	if A == B {
		fmt.Println(C)
	} else if A == C {
		fmt.Println(B)
	} else {
		fmt.Println(A)
	}
}
