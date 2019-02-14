// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 00:09:03 2019
//
package main

import "fmt"

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	ans := 1 << 60
	for i := 0; i < (1 << uint8(N)); i++ {
		n := 1
		for j := uint8(0); j < uint8(N); j++ {
			if i&(1<<j) != 0 {
				n *= 2
			} else {
				n += K
			}
		}
		if n < ans {
			ans = n
		}
	}

	fmt.Println(ans)
}
