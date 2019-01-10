// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:29:57 2019
//
package main

import "fmt"

func main() {
	var A, B int
	fmt.Scan(&A, &B)

	if A%3 == 0 || B%3 == 0 || (A+B)%3 == 0 {
		fmt.Println("Possible")
	} else {
		fmt.Println("Impossible")
	}
}
