// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 00:08:23 2019
//
package main

import "fmt"

type graph [][]int

func warshalFloyd(g graph) {
	for k := range g {
		for i := range g {
			for j := range g {
				if g[i][k]+g[k][j] < g[i][j] {
					g[i][j] = g[i][k] + g[k][j]
				}
			}
		}
	}
}

func permutation(n int) [][]int {
	if n == 0 {
		return [][]int{[]int{0}}
	}

	perm := permutation(n - 1)
	ret := [][]int{}
	for _, p := range perm {
		for i := 0; i <= len(p); i++ {
			q := make([]int, len(p))
			copy(q, p)
			ret = append(ret, append(q[:i], append([]int{n}, q[i:]...)...))
		}
	}
	return ret
}

func main() {
	var N, M, R int
	fmt.Scan(&N, &M, &R)

	rs := make([]int, R)
	for i := range rs {
		fmt.Scan(&rs[i])
	}

	g := make(graph, N)
	for i := range g {
		g[i] = make([]int, N)
		for j := range g[i] {
			if i != j {
				g[i][j] = 1 << 60
			}
		}
	}

	for i := 0; i < M; i++ {
		var a, b, c int
		fmt.Scan(&a, &b, &c)
		g[a-1][b-1] = c
		g[b-1][a-1] = c
	}

	warshalFloyd(g)
	ans := 1 << 60
	for _, p := range permutation(R - 1) {
		dist := 0
		for i := range p {
			if i+1 < len(p) {
				dist += g[rs[p[i]]-1][rs[p[i+1]]-1]
			}
		}
		if dist < ans {
			ans = dist
		}
	}

	fmt.Println(ans)
}
