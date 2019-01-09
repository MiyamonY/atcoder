// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 20:57:31 2019
//
package main

import "fmt"

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func main() {
	var N int
	fmt.Scan(&N)

	ans := 1
	for i := 0; i < N; i++ {
		var t int
		fmt.Scan(&t)
		ans = ans * (t / gcd(ans, t))
	}
	fmt.Println(ans)
}
