// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Feb  4 13:14:12 2019
//
package main

import "fmt"

type pair struct {
	x, y int
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func (p pair) dist(q pair) int {
	return abs(p.x-q.x) + abs(p.y-q.y)
}

func main() {
	var H, W, D int
	fmt.Scan(&H, &W, &D)

	nums := make([]pair, H*W)
	for i := 1; i <= H; i++ {
		for j := 1; j <= W; j++ {
			var n int
			fmt.Scan(&n)
			nums[n-1].x = i
			nums[n-1].y = j
		}
	}

	dists := make([]int, H*W)
	for i := 0; i < D; i++ {
		for j := i + D; j < len(dists); j += D {
			dists[j] = dists[j-D] + nums[j].dist(nums[j-D])
		}
	}

	var Q int
	fmt.Scan(&Q)
	for i := 0; i < Q; i++ {
		var l, r int
		fmt.Scan(&l, &r)
		fmt.Println(dists[r-1] - dists[l-1])
	}
}
