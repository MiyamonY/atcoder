// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 18:56:23 2018
//
package main

import "fmt"

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	if N == 1 && M == 1 {
		fmt.Println(1)
	} else if N == 1 || M == 1 {
		fmt.Println(N*M - 2)
	} else if N == 2 || M == 2 {
		fmt.Println(0)
	} else {
		fmt.Println((N - 2) * (M - 2))
	}
}
