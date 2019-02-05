// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Feb  5 11:13:15 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type route struct {
	to, n int
}

type graph [][]route

var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func dfs(g graph, dists []int, n, d int) bool {
	if dists[n] != -1<<60 {
		return dists[n] == d
	}

	dists[n] = d
	for i := range g[n] {
		r := g[n][i]
		if !dfs(g, dists, r.to, d+r.n) {
			return false
		}
	}
	return true
}

func main() {
	sc.Split(bufio.ScanWords)
	N, M := nextInt(), nextInt()

	g := make(graph, N)
	for i := 0; i < M; i++ {
		L, R, D := nextInt(), nextInt(), nextInt()
		g[L-1] = append(g[L-1], route{to: R - 1, n: D})
		g[R-1] = append(g[R-1], route{to: L - 1, n: -D})
	}

	dists := make([]int, N)
	for i := range dists {
		dists[i] = -1 << 60
	}

	for i := range dists {
		if dists[i] == -1<<60 {
			if !dfs(g, dists, i, 0) {
				fmt.Println("No")
				return
			}
		}
	}

	fmt.Println("Yes")
}
