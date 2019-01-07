// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 01:55:15 2019
//
package main

import "fmt"

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	board := make([]string, H)
	for i := range board {
		fmt.Scan(&board[i])
	}

	for i := range board {
		for j := range board[i] {
			if board[i][j] == '#' {
				fmt.Print("#")
			} else {
				num := 0
				for _, dx := range []int{-1, 0, 1} {
					for _, dy := range []int{-1, 0, 1} {
						if dx == 0 && dy == 0 {
							continue
						}
						x, y := i+dx, j+dy
						if 0 <= x && x < H && 0 <= y && y < W {
							if board[x][y] == '#' {
								num++
							}
						}
					}
				}
				fmt.Print(num)
			}
		}
		fmt.Println()
	}
}
