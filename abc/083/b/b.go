// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 17:24:32 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var N, A, B int
	fmt.Scan(&N, &A, &B)

	ans := 0
	for i := 1; i <= N; i++ {
		s := strconv.Itoa(i)

		sum := 0
		for i := range s {
			sum += int(s[i] - '0')
		}
		if A <= sum && sum <= B {
			ans += i
		}
	}
	fmt.Println(ans)
}
