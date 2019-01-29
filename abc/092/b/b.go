// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 23:15:31 2018
//
package main

import "fmt"

func main() {
	var N, D, X int
	fmt.Scan(&N, &D, &X)

	ans := 0
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		ans += (D-1)/a + 1
	}

	fmt.Println(ans + X)
}
