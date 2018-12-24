// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 24 16:07:06 2018
//
package main

import (
	"fmt"
	"sort"
)

type bridge struct {
	from, to int
}

type bridges []bridge

func (b bridges) Len() int {
	return len(b)
}

func (b bridges) Less(i, j int) bool {
	return b[i].to < b[j].to
}

func (b bridges) Swap(i, j int) {
	b[i], b[j] = b[j], b[i]
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	bs := make(bridges, M)
	for i := range bs {
		fmt.Scan(&bs[i].from, &bs[i].to)
	}
	sort.Sort(bs)

	last := 0
	ans := 0
	for _, b := range bs {
		if last < b.from {
			last = b.to - 1
			ans++
		}
	}
	fmt.Println(ans)
}
