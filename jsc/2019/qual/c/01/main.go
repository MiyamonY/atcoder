// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Aug 25 02:43:06 2019
//
package main

import "fmt"

const (
	left  = true
	right = false
	mod   = 1e9 + 7
)

func main() {
	var n int
	fmt.Scan(&n)
	var s string
	fmt.Scan(&s)

	if s[0] == 'W' || s[len(s)-1] == 'W' {
		fmt.Println(0)
		return
	}

	pos := make([]bool, 2*n)
	for i := range s {
		if i == 0 {
			pos[i] = left
			continue
		}

		if s[i] == s[i-1] {
			pos[i] = !pos[i-1]
		} else {
			pos[i] = pos[i-1]
		}
	}

	lefts, rights := 0, 0
	ans := 1
	for i := range pos {
		if pos[i] == left {
			lefts++
			continue
		}

		ans *= lefts - rights
		ans %= mod
		rights++
	}

	if lefts != rights || pos[len(pos)-1] != right {
		fmt.Println(0)
		return
	}

	for i := 1; i <= n; i++ {
		ans *= i
		ans %= mod
	}
	fmt.Println(ans)
}
