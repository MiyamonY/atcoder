// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 17:54:40 2019
//
package main

import (
	"errors"
	"fmt"
)

type graph [][]int

func warshalFloyd(g graph) (int, error) {
	g2 := make(graph, len(g))
	for i := range g2 {
		g2[i] = make([]int, len(g[i]))
		copy(g2[i], g[i])
	}

	for k := range g {
		for i := range g {
			for j := range g {
				if g[i][k]+g[k][j] < g[i][j] {
					return 0, errors.New("error")
				} else if i != k && k != j && g[i][k]+g[k][j] == g[i][j] {
					g2[i][j] = 0
				}
			}
		}
	}

	sum := 0
	for i := range g2 {
		for j := range g2[i] {
			sum += g2[i][j]
		}
	}

	return sum / 2, nil
}

func main() {
	var N int
	fmt.Scan(&N)

	routes := make(graph, N)
	for i := range routes {
		routes[i] = make([]int, N)
		for j := range routes[i] {
			fmt.Scan(&routes[i][j])
		}
	}

	if n, err := warshalFloyd(routes); err != nil {
		fmt.Println(-1)
	} else {
		fmt.Println(n)
	}
}
