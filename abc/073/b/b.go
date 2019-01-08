// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Tue Jan  8 23:49:30 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	ans := 0
	for i := 0; i < N; i++ {
		var l, r int
		fmt.Scan(&l, &r)
		ans += r - l + 1
	}

	fmt.Println(ans)
}
