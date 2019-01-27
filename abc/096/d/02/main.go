// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 17:52:26 2019
//
package main

import "fmt"

func isPrime(n int) bool {
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func main() {
	var N int
	fmt.Scan(&N)

	ans := make([]int, 0)
	for i := 5; i <= 55555; i += 5 {
		if isPrime(i + 1) {
			ans = append(ans, i+1)
		}
	}

	for i := 0; i < N; i++ {
		if i != 0 {
			fmt.Print(" ")
		}
		fmt.Print(ans[i])
	}
	fmt.Println()
}
