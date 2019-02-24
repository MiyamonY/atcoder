// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Jan 19 13:31:52 2019
//
package main

import "fmt"

type pair struct {
	x, y int
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	ps := make([]pair, M)
	for i := range ps {
		fmt.Scan(&ps[i].x, &ps[i].y)
	}

	for i := 1; i <= N; i++ {
		count := 0
		for j := range ps {
			if ps[j].x == i || ps[j].y == i {
				count++
			}
		}
		fmt.Println(count)
	}
}
