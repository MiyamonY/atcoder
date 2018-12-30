// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 23:19:11 2018
//
package main

import "fmt"

type pos struct {
	x, y int
}

func (p pos) dist(q pos) int {
	return abs(p.x-q.x) + abs(p.y-q.y)
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var H, W, D int

	_, _ = fmt.Scan(&H, &W, &D)

	arr := make([]pos, H*W+1)
	for i := 0; i < H; i++ {
		for j := 0; j < W; j++ {
			var a int
			_, _ = fmt.Scan(&a)
			arr[a].x = i + 1
			arr[a].y = j + 1
		}
	}

	dists := make([][]int, D)
	for i := 1; i <= H*W; i++ {
		if i <= D {
			dists[i%D] = append(dists[i%D], 0)
			if i == D {
				dists[i%D] = append(dists[i%D], 0)
			}
		} else {
			dists[i%D] = append(dists[i%D], arr[i].dist(arr[i-D]))
		}
	}

	for i := range dists {
		for j := range dists[i] {
			if j+1 < len(dists[i]) {
				dists[i][j+1] += dists[i][j]
			}
		}
	}

	var Q int
	_, _ = fmt.Scan(&Q)
	for i := 0; i < Q; i++ {
		var l, r int
		_, _ = fmt.Scan(&l, &r)
		fmt.Println(dists[r%D][r/D] - dists[l%D][l/D])
	}
}
