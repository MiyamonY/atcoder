// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 16:36:13 2018
//
package main

import (
	"fmt"
	"sort"
)

type cake struct {
	x, y, z int
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	cakes := make([]cake, N)
	for i := range cakes {
		fmt.Scan(&cakes[i].x, &cakes[i].y, &cakes[i].z)
	}

	ans := -1 << 60
	for ci := -1; ci <= 1; ci += 2 {
		for cj := -1; cj <= 1; cj += 2 {
			for ck := -1; ck <= 1; ck += 2 {
				arr := make([]int, N)
				for i, c := range cakes {
					arr[i] = ci*c.x + cj*c.y + ck*c.z
				}
				sort.Ints(arr)

				total := 0
				for _, a := range arr[len(arr)-M:] {
					total += a
				}
				ans = max(ans, total)
			}
		}
	}

	fmt.Println(ans)
}
