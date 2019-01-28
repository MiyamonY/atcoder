// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 23:39:17 2018
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
	var N, C int
	fmt.Scan(&N, &C)

	xs := make([]int, N)
	vs := make([]int, N)
	for i := range xs {
		fmt.Scan(&xs[i], &vs[i])
	}

	right := make([]int, N)
	for i := range right {
		right[i] = vs[i]
		if i > 0 {
			right[i] += right[i-1]
		}
	}

	left := make([]int, N)
	for i := range left {
		left[i] = vs[len(vs)-1-i]
		if i > 0 {
			left[i] += left[i-1]
		}
	}

	rightMax := make([]int, N)
	for i, r := range right {
		cal := r - xs[i]
		rightMax[i] = cal
		if i > 0 {
			rightMax[i] = max(rightMax[i], rightMax[i-1])
		}
	}

	leftMax := make([]int, N)
	for i, l := range left {
		cal := l - (C - xs[len(left)-1-i])
		leftMax[i] = cal
		if i > 0 {
			leftMax[i] = max(leftMax[i], leftMax[i-1])
		}
	}

	ans := max(0, rightMax[len(rightMax)-1], leftMax[len(leftMax)-1])
	for i := range rightMax {
		left := 0
		if i < len(rightMax)-1 {
			left = leftMax[N-i-2]
		}
		ans = max(ans, rightMax[i]-xs[i]+left)
	}

	for i := range leftMax {
		right := 0
		if i < len(leftMax)-1 {
			right = rightMax[N-i-2]
		}
		ans = max(ans, leftMax[i]-(C-xs[len(left)-1-i])+right)
	}

	fmt.Println(ans)
}
