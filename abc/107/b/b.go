// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Dec  6 02:10:17 2018
//
package main

import "fmt"

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	grid := make([]string, H)
	for i := range grid {
		fmt.Scan(&grid[i])
	}

	for i := range grid {
		hide := true
		for col := 0; col < W; col++ {
			if grid[i][col] == '#' {
				hide = false
			}
		}
		if hide {
			continue
		}

		for j := range grid[i] {
			hide := true
			for row := 0; row < H; row++ {
				if grid[row][j] == '#' {
					hide = false
				}
			}
			if hide {
				continue
			}
			fmt.Print(string(grid[i][j]))
		}
		fmt.Println()
	}
}
