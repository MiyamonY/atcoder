// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Wed Feb  6 11:16:01 2019
//
package main

import "fmt"

func main() {
	var N, Y int
	fmt.Scan(&N, &Y)

	for i := 0; i <= N; i++ {
		for j := 0; i+j <= N; j++ {
			k := N - (i + j)
			if 10000*i+5000*j+1000*k == Y {
				fmt.Println(i, j, k)
				return
			}
		}
	}
	fmt.Println(-1, -1, -1)
}
