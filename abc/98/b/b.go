// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Tue Dec 25 02:52:55 2018
//
package main

import "fmt"

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var N int
	var S string
	fmt.Scan(&N, &S)

	ans := 0
	for i := range S {
		first := S[:i]
		second := S[i:]

		m1, m2 := map[byte]bool{}, map[byte]bool{}
		for j := range first {
			m1[first[j]] = true
		}

		for j := range second {
			m2[second[j]] = true
		}

		tmp := 0
		for k := range m1 {
			if m2[k] {
				tmp++
			}
		}
		ans = max(ans, tmp)
	}
	fmt.Println(ans)
}
