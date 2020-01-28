// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jan 18 17:16:49 2020
//
package main

import "fmt"

var H, W int
var S []string

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func in(l, x, r int) bool {
	return l <= x && x < r
}

type pos struct {
	x int
	y int
}

func bfs(dists [][]int, x, y int) {
	dists[x][y] = 0
	q := []pos{pos{x, y}}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, p := range []pos{pos{-1, 0}, pos{1, 0}, pos{0, -1}, pos{0, 1}} {
			s := p.x + cur.x
			t := p.y + cur.y
			if in(0, s, H) && in(0, t, W) && dists[s][t] == -1 && S[s][t] != '#' {
				dists[s][t] = dists[cur.x][cur.y] + 1
				q = append(q, pos{s, t})
			}
		}
	}
}

func main() {
	fmt.Scan(&H, &W)
	S = make([]string, H)
	for i := range S {
		fmt.Scan(&S[i])
	}

	ans := 0
	for x := 0; x < H; x++ {
		for y := 0; y < W; y++ {
			dists := make([][]int, H)
			for i := range dists {
				dists[i] = make([]int, W)
				for j := range dists[i] {
					dists[i][j] = -1
				}
			}
			if S[x][y] == '.' {
				bfs(dists, x, y)
			}
			for i := range dists {
				for j := range dists[i] {
					ans = max(ans, dists[i][j])
				}
			}
		}
	}
	fmt.Println(ans)
}
