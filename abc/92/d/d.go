// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 23:34:29 2018
//
package main

import "fmt"

func main() {
	var A, B int
	fmt.Scan(&A, &B)

	grid := make([][]string, 100)
	for i := range grid {
		grid[i] = make([]string, 100)
		if i < 50 {
			for j := range grid[i] {
				grid[i][j] = "."
			}
		} else {
			for j := range grid[i] {
				grid[i][j] = "#"
			}
		}
	}

	for i := 0; i < B-1; i++ {
		a := 2 * (i / 50)
		b := (i % 50) * 2
		grid[a][b] = "#"
	}

	for i := 0; i < A-1; i++ {
		a := 2*(i/50) + 51
		b := (i % 50) * 2
		grid[a][b] = "."
	}

	fmt.Println(100, 100)
	for i := range grid {
		for j := range grid[i] {
			fmt.Print(grid[i][j])
		}
		fmt.Println()
	}
}
