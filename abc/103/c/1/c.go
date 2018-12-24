// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Dec  9 21:38:16 2018
//
package main

import "fmt"

func gcd(a, b int64) int64 {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func lcm(a, b int64) int64 {
	return a * b / gcd(a, b)
}

func main() {
	var N int
	fmt.Scan(&N)

	var ans int
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		ans += a - 1
	}

	fmt.Println(ans)
}
