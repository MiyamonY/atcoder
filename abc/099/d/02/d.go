// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 01:32:31 2019
//
package main

import "fmt"

func main() {
	var N, C int
	fmt.Scan(&N, &C)

	D := make([][]int, C)
	for i := range D {
		D[i] = make([]int, C)
	}

	for i := range D {
		for j := range D[i] {
			fmt.Scan(&D[i][j])
		}
	}

	m := [3]map[int]int{}
	for i := range m {
		m[i] = map[int]int{}
	}

	for i := 1; i <= N; i++ {
		for j := 1; j <= N; j++ {
			var a int
			fmt.Scan(&a)
			m[(i+j)%3][a-1]++
		}
	}

	ans := 1 << 60
	for i := 0; i < C; i++ {
		for j := 0; j < C; j++ {
			for k := 0; k < C; k++ {
				if i == j || j == k || k == i {
					continue
				}
				total := 0
				for key, v := range m[0] {
					total += D[key][i] * v
				}
				for key, v := range m[1] {
					total += D[key][j] * v
				}
				for key, v := range m[2] {
					total += D[key][k] * v
				}
				if total < ans {
					ans = total
				}
			}
		}
	}
	fmt.Println(ans)
}
