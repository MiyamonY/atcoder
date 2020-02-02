// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 15:14:37 2020
//
package main

import "fmt"

const MOD = 1e9 + 7

var H int

func main() {
	fmt.Scan(&H)

	n := 1
	ans := 0
	for i := 1; !(H < i && i <= 2*H); i *= 2 {
		ans += n
		n *= 2
	}
	fmt.Println(ans)
}
