// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 15:33:41 2018
//
package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)

	var hasA, hasB, hasC bool
	for i := range S {
		if S[i] == 'a' {
			hasA = true
		} else if S[i] == 'b' {
			hasB = true
		} else if S[i] == 'c' {
			hasC = true
		}
	}

	if hasA && hasB && hasC {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
