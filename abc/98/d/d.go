// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Tue Dec 25 03:29:25 2018
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	ans, sum, l := 0, 0, 0
	for r, a := range arr {
		for sum&a != 0 && l <= r {
			sum &= ^arr[l]
			l++
		}
		sum |= a
		ans += r - l + 1
	}

	fmt.Println(ans)
}
