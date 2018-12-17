// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 21:19:54 2018
//
package main

import "fmt"

func mid(n int64) int64 {
	var ret int64 = 1
	for i := int64(0); i < n; i++ {
		ret = 2*ret + 1
	}
	return ret
}

func num(n int64) int64 {
	if n == 0 {
		return 1
	}

	return 2*num(n-1) + 1
}

func calc(n, x int64) int64 {
	if x == 0 {
		return 0
	}

	if n == 0 {
		return 1
	}

	m := mid(n)
	if x >= m {
		return 1 + num(n-1) + calc(n-1, x-m)
	}

	return calc(n-1, x-1)
}

func main() {
	var N, X int64
	fmt.Scan(&N, &X)

	fmt.Println(calc(N, X))
}
