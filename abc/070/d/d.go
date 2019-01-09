// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 20:59:02 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
)

type route struct {
	to, weight int
}

type graph [][]route

var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}

var depths []int

func dfs(n int, g graph, weight int) {
	depths[n] = weight
	for i := range g[n] {
		to, w := g[n][i].to, g[n][i].weight
		if depths[to] == -1 && w > 0 {
			dfs(to, g, weight+w)
		}
	}
}

func main() {
	var N, Q, K int
	fmt.Sscan(nextLine(), &N)

	g := make(graph, N)

	for i := 0; i < N-1; i++ {
		var a, b, c int
		fmt.Sscan(nextLine(), &a, &b, &c)
		g[a-1] = append(g[a-1], route{to: b - 1, weight: c})
		g[b-1] = append(g[b-1], route{to: a - 1, weight: c})
	}

	fmt.Sscan(nextLine(), &Q, &K)

	depths = make([]int, len(g))
	for i := range depths {
		depths[i] = -1
	}
	dfs(K-1, g, 0)

	for i := 0; i < Q; i++ {
		var x, y int
		fmt.Sscan(nextLine(), &x, &y)
		fmt.Println(depths[x-1] + depths[y-1])
	}
}
