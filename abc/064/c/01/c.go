// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan 16 19:19:02 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	m := map[int]int{}
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		if a < 400 {
			m[0]++
		} else if a < 800 {
			m[400]++
		} else if a < 1200 {
			m[800]++
		} else if a < 1600 {
			m[1200]++
		} else if a < 2000 {
			m[1600]++
		} else if a < 2400 {
			m[2000]++
		} else if a < 2800 {
			m[2400]++
		} else if a < 3200 {
			m[2800]++
		} else {
			m[3200]++
		}
	}

	min := 0
	max := 0
	for k, v := range m {
		if k == 3200 {
			max += v
		} else {
			min++
			max++
		}
	}

	if min == 0 {
		min++
	}

	fmt.Println(min, max)
}
