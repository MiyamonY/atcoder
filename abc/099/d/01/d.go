// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Tue Dec 25 02:07:30 2018
//
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	var N, C int
	fmt.Scan(&N, &C)

	D := make([][]int, C)
	for i := range D {
		D[i] = make([]int, C)
		for j := range D[i] {
			fmt.Scan(&D[i][j])
		}
	}

	m1 := map[int]int{}
	m2 := map[int]int{}
	m3 := map[int]int{}
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			var c int
			fmt.Scan(&c)
			c--
			switch (i + j) % 3 {
			case 0:
				m1[c]++
			case 1:
				m2[c]++
			case 2:
				m3[c]++
			}
		}
	}

	ans := 1 << 60
	for ci := 0; ci < C; ci++ {
		for cj := 0; cj < C; cj++ {
			for ck := 0; ck < C; ck++ {
				if ci == cj || cj == ck || ck == ci {
					continue
				}
				tmp := 0
				for k, v := range m1 {
					tmp += v * D[k][ci]
				}
				for k, v := range m2 {
					tmp += v * D[k][cj]
				}
				for k, v := range m3 {
					tmp += v * D[k][ck]
				}
				ans = min(ans, tmp)
			}
		}
	}

	fmt.Println(ans)
}
