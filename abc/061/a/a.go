// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Jan 19 13:29:47 2019
//
package main

import "fmt"

func main() {
	var A, B, C int
	fmt.Scan(&A, &B, &C)
	if A <= C && C <= B {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
