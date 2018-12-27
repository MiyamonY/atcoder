///
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 13:18:28 2018
//
package main

import (
	"fmt"
)

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	graph := make([]string, H)
	for i := range graph {
		fmt.Scan(&graph[i])
	}

	for i := range graph {
		for j := range graph[i] {
			if graph[i][j] == '#' {
				valid := false
				for _, d := range []int{-1, 1} {
					if 0 <= i+d && i+d < H && graph[i+d][j] == '#' {
						valid = true
					}
					if 0 <= j+d && j+d < W && graph[i][j+d] == '#' {
						valid = true
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
