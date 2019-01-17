// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 11:41:22 2019
//
package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)

	for i := range S {
		for j := i + 1; j < len(S); j++ {
			if S[i] == S[j] {
				fmt.Println("no")
				return
			}
		}
	}
	fmt.Println("yes")
}
