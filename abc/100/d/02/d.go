// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Jan 25 20:32:42 2019
//
package main

import (
	"fmt"
	"sort"
)

type cake struct {
	x, y, z int
}

func (c cake) val() int {
	return c.x + c.y + c.z
}

type cakes []cake

func (cs cakes) Len() int {
	return len(cs)
}

func (cs cakes) Swap(i, j int) {
	cs[i], cs[j] = cs[j], cs[i]
}

func (cs cakes) Less(i, j int) bool {
	return cs[i].val() > cs[j].val()
}

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

var N, M int

func main() {
	fmt.Scan(&N, &M)

	cs := [8]cakes{}
	for i := range cs {
		cs[i] = make(cakes, N)
	}

	for i := 0; i < N; i++ {
		var x, y, z int
		fmt.Scan(&x, &y, &z)
		cs[0][i] = cake{x: x, y: y, z: z}
		cs[1][i] = cake{x: x, y: y, z: -z}
		cs[2][i] = cake{x: x, y: -y, z: z}
		cs[3][i] = cake{x: x, y: -y, z: -z}
		cs[4][i] = cake{x: -x, y: y, z: z}
		cs[5][i] = cake{x: -x, y: y, z: -z}
		cs[6][i] = cake{x: -x, y: -y, z: z}
		cs[7][i] = cake{x: -x, y: -y, z: -z}
	}

	ans := 0
	for i := range cs {
		sort.Sort(cs[i])
		sum := 0
		for j := 0; j < M; j++ {
			sum += cs[i][j].val()
		}
		ans = max(ans, sum)
	}

	fmt.Println(ans)
}
