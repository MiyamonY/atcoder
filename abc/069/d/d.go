// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 09:47:06 2019
//
package main

import "fmt"

func main() {
	var H, W, N int
	fmt.Scan(&H, &W, &N)
	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	board := make([][]int, H)
	for i := range board {
		board[i] = make([]int, W)
	}

	h, w := 0, 0
	for i := range arr {
		for arr[i] > 0 {
			board[h][w] = i + 1
			w++
			if w == W {
				h++
				w = 0
			}
			arr[i]--
		}
	}

	for i := range board {
		if i%2 == 0 {
			for j := range board[i] {
				if j != 0 {
					fmt.Print(" ")
				}
				fmt.Print(board[i][j])
			}
		} else {
			for j := len(board[i]) - 1; j >= 0; j-- {
				if j != len(board[i])-1 {
					fmt.Print(" ")
				}
				fmt.Print(board[i][j])
			}
		}
		fmt.Println()
	}
}
