// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 17:47:15 2019
//
package main

import "fmt"

type pair struct {
	x, y int
}

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	board := make([]string, H)
	for i := range board {
		fmt.Scan(&board[i])
	}

	for i := 0; i < H; i++ {
		for j := 0; j < W; j++ {
			if board[i][j] == '#' {
				valid := false
				for _, p := range []pair{{0, -1}, {0, 1}, {-1, 0}, {1, 0}} {
					x, y := i+p.x, j+p.y
					if 0 <= x && x < H && 0 <= y && y < W {
						if board[x][y] == '#' {
							valid = true
						}
					}
				}
				if !valid {
					fmt.Println("No")
					return
				}
			}
		}
	}

	fmt.Println("Yes")
}
