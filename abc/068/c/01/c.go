// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:07:30 2019
//
package main

import "fmt"

type graph [][]int

func dfs(from int, to int, g graph, n int) bool {
	if n == 0 {
		for i := range g[from] {
			if g[from][i] == to {
				return true
			}
		}
		return false
	}

	for i := range g[from] {
		if dfs(g[from][i], to, g, n-1) {
			return true
		}
	}
	return false
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	g := make(graph, N+1)
	for i := 0; i < M; i++ {
		var a, b int
		fmt.Scan(&a, &b)
		g[a] = append(g[a], b)
		g[b] = append(g[b], a)
	}

	if dfs(1, N, g, 1) {
		fmt.Println("POSSIBLE")
	} else {
		fmt.Println("IMPOSSIBLE")
	}
}
