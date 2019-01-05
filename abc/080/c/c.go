// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 16:18:20 2019
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
	var N int
	fmt.Scan(&N)
	f := make([][]int, N)
	for i := range f {
		f[i] = make([]int, 10)
		for j := range f[i] {
			fmt.Scan(&f[i][j])
		}
	}

	p := make([][]int, N)
	for i := range p {
		p[i] = make([]int, 11)
		for j := range p[i] {
			fmt.Scan(&p[i][j])
		}
	}

	ans := -1 << 60
	for i := 1; i < 1<<10; i++ {
		profit := 0
		for n := 0; n < N; n++ {
			same := 0
			for j := 0; j < 10; j++ {
				if i&(1<<uint8(j)) != 0 {
					same += f[n][j]
				}
			}
			profit += p[n][same]
		}
		ans = max(ans, profit)
	}

	fmt.Println(ans)
}
