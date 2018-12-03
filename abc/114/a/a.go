// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sun Dec  2 20:57:55 2018
//
package main

import "fmt"

var x int

func main() {
	fmt.Scan(&x)
	if x == 7 || x == 5 || x == 3 {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
