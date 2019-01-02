// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Tue Jan  1 01:13:11 2019
//
package main

import "fmt"

func main() {
	var X, A, B int
	fmt.Scan(&X, &A, &B)
	fmt.Println((X - A) % B)
}
