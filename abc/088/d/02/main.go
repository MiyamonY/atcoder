// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Feb  4 14:48:56 2019
//
package main

import "fmt"

type pair struct {
	x, y int
}

var H, W int

func in(x, y int) bool {
	return 0 <= x && x < H && 0 <= y && y < W
}
func dfs(g []string, dists [][]int) {
	dists[0][0] = 0
	q := []pair{pair{0, 0}}

	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		for _, d := range []pair{pair{1, 0}, pair{-1, 0}, pair{0, 1}, pair{0, -1}} {
			x, y := p.x+d.x, p.y+d.y
			if in(x, y) && g[x][y] == '.' && dists[x][y] == -1 {
				dists[x][y] = dists[p.x][p.y] + 1
				q = append(q, pair{x, y})
			}
		}
	}
}

func main() {
	fmt.Scan(&H, &W)

	g := make([]string, H)
	blacks := 0
	for i := range g {
		fmt.Scan(&g[i])
		for j := range g[i] {
			if g[i][j] == '#' {
				blacks++
			}
		}
	}

	dists := make([][]int, H)
	for i := range dists {
		dists[i] = make([]int, W)
		for j := range dists[i] {
			dists[i][j] = -1
		}
	}

	dfs(g, dists)
	if dists[H-1][W-1] == -1 {
		fmt.Println(-1)
	} else {
		fmt.Println(H*W - dists[H-1][W-1] - 1 - blacks)
	}
}
