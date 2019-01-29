// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Jan 29 11:07:21 2019
//
package main

import "fmt"

func main() {
	var A, B int
	fmt.Scan(&A, &B)

	board := make([][]string, 100)
	for i := range board {
		board[i] = make([]string, 100)
		for j := range board[i] {
			if i < 50 {
				board[i][j] = "#"
			} else {
				board[i][j] = "."
			}
		}
	}

	h := 0
	w := 0
	for i := 0; i < A-1; i++ {
		board[h][w] = "."
		w += 2
		if w >= 100 {
			w = 0
			h += 2
		}
	}

	h = 51
	w = 0
	for i := 0; i < B-1; i++ {
		board[h][w] = "#"
		w += 2
		if w >= 100 {
			w = 0
			h += 2
		}
	}

	fmt.Println(100, 100)
	for i := range board {
		for j := range board[i] {
			fmt.Print(board[i][j])
		}
		fmt.Println()
	}
}
