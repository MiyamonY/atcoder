// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec 22 01:53:10 2018
//
package main

import "fmt"

type line struct {
	from, to, weight int
}

func pow2(n int) int {
	if n == 0 {
		return 1
	}
	return 2 * pow2(n-1)
}

func div2Count(n int) int {
	ans := 0
	for n > 1 {
		n /= 2
		ans++
	}
	return ans
}

func main() {
	var L int
	fmt.Scan(&L)

	n := div2Count(L) + 1
	lines := make([]line, 0)
	for i := 0; i < n; i++ {
		if i+1 < n {
			lines = append(lines, line{from: i + 1, to: i + 2, weight: 0})
			lines = append(lines, line{from: i + 1, to: i + 2, weight: pow2(i)})
		}
	}

	rem := L - pow2(n-1)
	weight := pow2(n - 1)
	for rem > 0 {
		a := div2Count(rem)
		rem -= pow2(a)
		lines = append(lines, line{from: a + 1, to: n, weight: weight})
		weight += pow2(a)
	}

	fmt.Println(n, len(lines))
	for _, l := range lines {
		fmt.Println(l.from, l.to, l.weight)
	}
}
