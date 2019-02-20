// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:45:19 2019
//
package main

import (
	"fmt"
)

type graph struct {
	routes  [][]int
	visited []bool
}

func (g graph) addRoute(from, to int) {
	g.routes[from] = append(g.routes[from], to)
	g.routes[to] = append(g.routes[to], from)
}

func (g graph) dfs(from int, dist int, dists []int) {
	dists[from] = dist
	g.visited[from] = true

	for i := range g.routes[from] {
		to := g.routes[from][i]
		if !g.visited[to] {
			g.dfs(to, dist+1, dists)
		}
	}
}

func (g graph) dists(from int) []int {
	dists := make([]int, len(g.routes))
	for i := range g.visited {
		g.visited[i] = false
	}

	g.dfs(from, 0, dists)
	return dists
}

func newGraph(n int) graph {
	g := graph{}
	g.routes = make([][]int, n)
	g.visited = make([]bool, n)

	return g
}

func main() {
	var N int
	fmt.Scan(&N)

	g := newGraph(N + 1)
	for i := 0; i < N-1; i++ {
		var a, b int
		fmt.Scan(&a, &b)
		g.addRoute(a, b)
	}
	dists1 := g.dists(1)
	distsN := g.dists(N)

	fennec, sunuke := 0, 0
	for i := 1; i <= N; i++ {
		if dists1[i] <= distsN[i] {
			fennec++
		} else {
			sunuke++
		}
	}

	if fennec > sunuke {
		fmt.Println("Fennec")
	} else {
		fmt.Println("Snuke")
	}
}
