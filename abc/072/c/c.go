// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 00:33:54 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	counts := make([]int, 1e5)
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		counts[a]++
	}
	ans := 0
	for i := 0; i+2 < len(counts); i++ {
		n := counts[i] + counts[i+1] + counts[i+2]
		if ans < n {
			ans = n
		}
	}
	fmt.Println(ans)
}
