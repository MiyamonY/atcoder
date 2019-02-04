// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Fri Feb  1 19:23:56 2019
//
package main

import "fmt"

func main() {
	var N, M int
	fmt.Scan(&N, &M)
	if N == 1 && M == 1 {
		fmt.Println(1)
	} else if N == 1 || M == 1 {
		if N > 2 || M > 2 {
			fmt.Println(N*M - 2)
		} else {
			fmt.Println(0)
		}
	} else if N == 2 || M == 2 {
		fmt.Println(0)
	} else {
		fmt.Println(N*M - 2*N - 2*M + 4)
	}
}
