// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 23:01:59 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
)

var N, K int
var board [][]int

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

func countRegion(y, x int) int {
	limY, limX := y+K-1, x+K-1
	limY = max(limY, 0)
	limY = min(limY, len(board)-1)
	limX = max(limX, 0)
	limX = min(limX, len(board[0])-1)

	minLimY, minLimX := y-1, x-1
	minLimY = max(minLimY, 0)
	minLimY = min(minLimY, len(board)-1)
	minLimX = max(minLimX, 0)
	minLimX = min(minLimX, len(board[0])-1)

	return board[limY][limX] - board[limY][minLimX] - board[minLimY][limX] + board[minLimY][minLimX]
}

var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}

func main() {
	fmt.Scan(&N, &K)

	board = make([][]int, 2*K+2)
	for i := range board {
		board[i] = make([]int, 2*K+2)
	}

	for i := 0; i < N; i++ {
		var x, y int
		var c string

		fmt.Sscan(nextLine(), &x, &y, &c)
		if c == "B" {
			y += K
		}
		board[(y%(2*K))+1][(x%(2*K))+1]++
	}

	for i := range board {
		for j := range board[i] {
			if j+1 < len(board[i]) {
				board[i][j+1] += board[i][j]
			}
		}
	}

	for i := range board {
		for j := range board[i] {
			if i+1 < len(board[i]) {
				board[i+1][j] += board[i][j]
			}
		}
	}

	ans := 0
	for i := 1; i < len(board)-1; i++ {
		for j := 1; j < len(board[i])-1; j++ {
			count := countRegion(i, j)

			count += countRegion(i+K, j+K)
			count += countRegion(i+K, j-K)
			count += countRegion(i-K, j+K)
			count += countRegion(i-K, j-K)

			count += countRegion(i, j-2*K)
			count += countRegion(i-2*K, j)
			count += countRegion(i-2*K, j-2*K)

			ans = max(ans, count)
		}
	}

	fmt.Println(ans)
}
