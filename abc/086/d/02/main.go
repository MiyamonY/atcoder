// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Feb  5 12:38:52 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type graph [][]int

var N, K int

var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func nextString() string {
	sc.Scan()
	return sc.Text()
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func in(x, y int) bool {
	return 0 <= x && x < 2*K && 0 <= y && y < 2*K
}

func inRect(x, y int) bool {
	return in(x, y) || in(x+K-1, y+K-1) || in(x-1, y+K-1) || in(x+K-1, y-1)
}

func rect(x, y int, g graph) int {
	if !inRect(x, y) {
		return 0
	}

	tx := min(x+K-1, len(g)-1)
	ty := min(y+K-1, len(g)-1)
	nums := g[tx][ty]
	if in(x-1, y-1) {
		nums += g[x-1][y-1]
	}

	if in(x-1, ty) {
		nums -= g[x-1][ty]
	}

	if in(tx, y-1) {
		nums -= g[tx][y-1]
	}
	return nums
}

func main() {
	sc.Split(bufio.ScanWords)

	N, K = nextInt(), nextInt()

	g := make(graph, 2*K)
	for i := range g {
		g[i] = make([]int, 2*K)
	}

	for i := 0; i < N; i++ {
		x, y := nextInt(), nextInt()
		s := nextString()
		x %= 2 * K
		y %= 2 * K
		if s == "B" {
			x += K
			x %= 2 * K
		}
		g[x][y]++
	}

	for i := range g {
		for j := range g[i] {
			if j > 0 {
				g[i][j] += g[i][j-1]
			}
		}
	}

	for i := range g[0] {
		for j := range g {
			if j > 0 {
				g[j][i] += g[j-1][i]
			}
		}
	}

	ans := 0
	for i := 0; i < len(g); i++ {
		for j := 0; j < len(g); j++ {
			nums := rect(i, j, g)
			nums += rect(i-K, j+K, g)
			nums += rect(i+K, j-K, g)
			nums += rect(i+K, j+K, g)
			nums += rect(i-K, j-K, g)
			nums += rect(i-2*K, j, g)
			nums += rect(i, j-2*K, g)
			nums += rect(i-2*K, j-2*K, g)
			ans = max(ans, nums)
		}
	}

	fmt.Println(ans)
}
