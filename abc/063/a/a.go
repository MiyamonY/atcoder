// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 11:39:12 2019
//
package main

import "fmt"

func main() {
	var A, B int
	fmt.Scan(&A, &B)
	if A+B < 10 {
		fmt.Println(A + B)
	} else {
		fmt.Println("error")
	}
}
