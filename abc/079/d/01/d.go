// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:15:43 2019
//
package main

import "fmt"

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	graph := make([][]int, 10)
	for i := range graph {
		graph[i] = make([]int, 10)
		for j := range graph[i] {
			fmt.Scan(&graph[i][j])
		}
	}

	for k := 0; k < 10; k++ {
		for i := 0; i < 10; i++ {
			for j := 0; j < 10; j++ {
				if graph[i][k]+graph[k][j] < graph[i][j] {
					graph[i][j] = graph[i][k] + graph[k][j]
				}
			}
		}
	}

	ans := 0
	for i := 0; i < H; i++ {
		for j := 0; j < W; j++ {
			var a int
			fmt.Scan(&a)
			if a != -1 {
				ans += graph[a][1]
			}
		}
	}
	fmt.Println(ans)
}
