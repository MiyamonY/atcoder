// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 16:28:50 2019
//
package main

import (
	"fmt"
	"sort"
)

type point struct {
	x, y int
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)
	points := make([]point, N)

	for i := range points {
		fmt.Scan(&points[i].x, &points[i].y)
	}

	xs := make([]int, N)
	ys := make([]int, N)
	for i := range xs {
		xs[i] = points[i].x
		ys[i] = points[i].y
	}
	sort.Ints(xs)
	sort.Ints(ys)

	ans := 1 << 62
	for _, xlower := range xs {
		for _, xupper := range xs {
			for _, ylower := range ys {
				for _, yupper := range ys {
					num := 0
					for _, p := range points {
						x, y := p.x, p.y
						if xlower < xupper && ylower < yupper && xlower <= x && x <= xupper && ylower <= y && y <= yupper {
							num++
						}
					}
					if num >= K {
						area := (xupper - xlower) * (yupper - ylower)
						if area < ans {
							ans = area
						}
					}
				}
			}
		}
	}

	fmt.Println(ans)
}
