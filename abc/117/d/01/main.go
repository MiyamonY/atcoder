// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  3 21:15:17 2019
//
package main

import (
	"fmt"
	"sort"
)

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	bits := [40]int{}
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		for j := uint8(0); j < 40; j++ {
			if a&(1<<j) != 0 {
				bits[j]++
			}
		}
	}

	ans := 0
	count := 0
	for j := len(bits) - 1; j >= 0; j-- {
		n1 := bits[j]
		n0 := N - n1
		m := 1 << uint8(j)
		if n0 > n1 && (count+m) <= K {
			ans += n0 * m
			count += m
		} else {
			ans += n1 * m
		}
	}
	fmt.Println(ans)
}
