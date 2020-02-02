// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 10:56:11 2020
//
package main

import "fmt"

var a, b int

func main() {
	fmt.Scan(&a, &b)
	if a < b {
		for i := 0; i < b; i++ {
			fmt.Print(a)
		}
		fmt.Println()
	} else {
		for i := 0; i < a; i++ {
			fmt.Print(b)
		}
		fmt.Println()
	}
}
