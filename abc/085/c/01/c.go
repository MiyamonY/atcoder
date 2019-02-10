// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 12:25:13 2019
//
package main

import (
	"fmt"
)

func main() {
	var N, Y int
	fmt.Scan(&N, &Y)

	for i := 0; i <= N; i++ {
		for j := 0; j <= N; j++ {
			k := N - i - j
			if k >= 0 && Y == 1000*i+5000*j+10000*k {
				fmt.Println(k, j, i)
				return
			}
		}
	}

	fmt.Println(-1, -1, -1)
}
