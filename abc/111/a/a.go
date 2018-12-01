// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 15:38:26 2018
//
package main

import "fmt"

var s string

func main() {
	fmt.Scan(&s)

	for _, c := range s {
		if c == '1' {
			fmt.Print(9)
		} else {
			fmt.Print(1)
		}
	}
	fmt.Println()
}
