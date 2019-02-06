// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Wed Feb  6 11:42:59 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	cs := make([]int, N-1)
	ss := make([]int, N-1)
	fs := make([]int, N-1)

	for i := range cs {
		fmt.Scan(&cs[i], &ss[i], &fs[i])
	}

	for i := range cs {
		t := 0
		for j := i; j < len(cs); j++ {
			c, s, f := cs[j], ss[j], fs[j]
			if t <= s {
				t = s
			} else if t > s {
				n := (t - s + f - 1) / f
				t = s + n*f
			}
			t += c
		}
		fmt.Println(t)
	}
	fmt.Println(0)
}
