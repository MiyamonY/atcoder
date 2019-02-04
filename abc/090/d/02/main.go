// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Feb  4 12:11:39 2019
//
package main

import "fmt"

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	ans := 0
	for b := 1; b <= N; b++ {
		if b-1 < K {
			continue
		}
		num := (b - 1) - K + 1
		ans += (N / b) * num

		rem := (N % b) - K + 1
		if rem > 0 {
			ans += rem
		}
	}

	if K == 0 {
		ans -= N
	}
	fmt.Println(ans)
}
