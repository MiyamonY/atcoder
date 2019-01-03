// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 15:38:27 2019
//
package main

import "fmt"

func main() {
	var A, B int
	var s string

	fmt.Scan(&A, &B, &s)

	for i := 0; i < A; i++ {
		if !('0' <= s[i] && s[i] <= '9') {
			fmt.Println("No")
			return
		}
	}

	if s[A] != '-' {
		fmt.Println("No")
		return
	}

	for i := A + 1; i < len(s); i++ {
		if !('0' <= s[i] && s[i] <= '9') {
			fmt.Println("No")
			return
		}
	}

	fmt.Println("Yes")
}
