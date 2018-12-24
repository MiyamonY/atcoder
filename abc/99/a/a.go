// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 23:45:41 2018
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	if N >= 1000 {
		fmt.Println("ABD")
		return
	}
	fmt.Println("ABC")
}
