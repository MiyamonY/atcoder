// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Dec  6 02:18:23 2018
//
package main

import (
	"fmt"
	"sort"
)

var N, K int

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func timeMin(first []int, second []int) int {
	timeMin := 1000000000
	for i := 1; i <= K; i++ {
		if i <= len(first) && K-i <= len(second) {
			var time int
			if K-i-1 < 0 {
				time = first[i-1]
			} else {
				time = 2*first[i-1] + second[K-i-1]
			}

			timeMin = min(time, timeMin)
		}
	}

	return timeMin
}

func main() {
	fmt.Scan(&N, &K)

	var left, right []int
	for i := 0; i < N; i++ {
		var x int
		fmt.Scan(&x)
		if x < 0 {
			left = append(left, -x)
		} else {
			right = append(right, x)
		}
	}
	sort.Ints(left)

	fmt.Println(min(timeMin(left, right), timeMin(right, left)))
}
