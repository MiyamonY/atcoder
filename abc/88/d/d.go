// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 31 16:35:13 2018
//
package main

import "fmt"

var graph []string
var H, W int

type pos struct {
	x, y int
}

func (p pos) isInRange() bool {
	return 0 <= p.x && p.x < H && 0 <= p.y && p.y < W
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func dfs(a pos, dists [][]int) {
	for _, d := range []pos{{x: 1, y: 0}, {x: 0, y: 1}, {x: -1, y: 0}, {x: 0, y: -1}} {
		p := pos{x: a.x + d.x, y: a.y + d.y}
		if p.isInRange() && graph[p.x][p.y] == '.' {
			dist := dists[a.x][a.y] + 1
			if dist < dists[p.x][p.y] {
				dists[p.x][p.y] = dist
				dfs(p, dists)
			}
		}
	}
}

func countWhites() int {
	ret := 0
	for i := range graph {
		for j := range graph[i] {
			if graph[i][j] == '.' {
				ret++
			}
		}
	}
	return ret
}

func main() {
	fmt.Scan(&H, &W)

	graph = make([]string, H)
	for i := range graph {
		fmt.Scan(&graph[i])
	}

	dists := make([][]int, H)
	for i := range dists {
		dists[i] = make([]int, W)
		for j := range dists[i] {
			dists[i][j] = 1 << 30
		}
	}
	dists[0][0] = 0

	dfs(pos{x: 0, y: 0}, dists)
	if dists[H-1][W-1] == 1<<30 {
		fmt.Println(-1)
		return
	}
	fmt.Println(countWhites() - dists[H-1][W-1] - 1)
}
