// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Feb  5 11:09:44 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	a1 := make([]int, N)
	for i := range a1 {
		fmt.Scan(&a1[i])
	}

	a2 := make([]int, N)
	for i := range a2 {
		fmt.Scan(&a2[i])
	}

	for i := range a1 {
		if i+1 < len(a1) {
			a1[i+1] += a1[i]
			a2[i+1] += a2[i]
		}
	}

	ans := 0
	for i := 0; i < len(a1); i++ {
		var n int
		if i == 0 {
			n = a1[0] + a2[len(a2)-1]
		} else {
			n = a1[i] + a2[len(a2)-1] - a2[i-1]
		}
		if ans < n {
			ans = n
		}
	}
	fmt.Println(ans)
}
