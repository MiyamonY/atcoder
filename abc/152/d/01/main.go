// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 11:06:47 2020
//
package main

import "fmt"

type pair struct {
	x int
	y int
}

var N int

func top(n int) int {
	m := 1
	for {
		if n/(m*10) == 0 {
			return n / m
		}
		m *= 10
	}
}

func main() {
	fmt.Scan(&N)
	m := map[pair]int{}

	for i := 1; i <= N; i++ {
		m[pair{top(i), i % 10}]++
	}

	ans := 0
	for p, x := range m {
		y := m[pair{p.y, p.x}]
		ans += x * y
	}

	fmt.Println(ans)
}
