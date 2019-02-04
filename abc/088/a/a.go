// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Mon Dec 31 13:14:56 2018
//
package main

import "fmt"

func main() {
	var N, A int
	fmt.Scan(&N, &A)
	if N%500 <= A {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
