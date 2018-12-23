// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 00:45:25 2018
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	ans := 0
	base := 1
	for i := 0; N != 0; i++ {
		if N%(base*2) != 0 {
			ans += 1 << uint8(i)
			N -= base
		}
		base *= -2
	}

	fmt.Printf("%b\n", ans)
}
