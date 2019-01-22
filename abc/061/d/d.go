// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan 19 14:14:34 2019
//
package main

import (
	"fmt"
)

type route struct {
	to, weight int
}

type graph [][]route

func main() {
	var N, M int
	fmt.Scan(&N, &M)
	g := make(graph, N)

	for i := 0; i < M; i++ {
		var a, b, c int
		fmt.Scan(&a, &b, &c)
		a--
		b--
		g[a] = append(g[a], route{to: b, weight: -c})
	}

	dists := make([]int, N)
	for i := range dists {
		dists[i] = 1 << 60
	}
	dists[0] = 0

	for rep := 0; rep < N-1; rep++ {
		for i := range g {
			if dists[i] == 1<<60 {
				continue
			}

			for _, r := range g[i] {
				t, w := r.to, r.weight
				d := dists[i] + w
				if d < dists[t] {
					dists[t] = d
				}
			}
		}
	}

	ans := -dists[N-1]

	negatives := make([]bool, N)
	for rep := 0; rep < N; rep++ {
		for i := range g {
			if dists[i] == 1<<60 {
				continue
			}
			for _, r := range g[i] {
				t, w := r.to, r.weight
				d := dists[i] + w
				if d < dists[t] {
					dists[t] = d
					negatives[t] = true
				}

				if negatives[i] == true {
					negatives[t] = true
				}
			}
		}
	}

	if negatives[N-1] {
		fmt.Println("inf")
	} else {
		fmt.Println(ans)
	}
}
