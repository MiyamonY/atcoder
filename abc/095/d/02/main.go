// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 18:10:19 2019
//
package main

import (
	"fmt"
	"sort"
)

type pair struct {
	x, v int
}

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func main() {
	var N, C int
	fmt.Scan(&N, &C)

	ps := make([]pair, N)
	for i := range ps {
		fmt.Scan(&ps[i].x, &ps[i].v)
	}

	rV := make([]int, N)
	for i := range ps {
		if i == 0 {
			rV[i] = ps[i].v
		} else {
			rV[i] = rV[i-1] + ps[i].v
		}
	}

	for i := range rV {
		rV[i] -= ps[i].x
		if i > 0 {
			rV[i] = max(rV[i], rV[i-1])
		}
	}

	lV := make([]int, N)
	for i := range ps {
		if i == 0 {
			lV[i] = ps[len(ps)-1-i].v
		} else {
			lV[i] = lV[i-1] + ps[len(ps)-1-i].v
		}
	}

	for i := range lV {
		lV[i] -= C - ps[len(ps)-1-i].x
		if i > 0 {
			lV[i] = max(lV[i], lV[i-1])
		}
	}

	ans := max(0, rV[len(rV)-1], lV[len(lV)-1])
	for i := range rV {
		if i < len(rV)-1 {
			ans = max(ans, rV[i]-ps[i].x+lV[len(lV)-2-i], lV[i]-(C-ps[len(ps)-i-1].x)+rV[len(rV)-2-i])
		}
	}

	fmt.Println(ans)
}
