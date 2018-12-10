// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec  9 22:40:43 2018
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

func (b bridges) Swap(i, j int) {
	b[i], b[j] = b[j], b[i]
}

func (b bridges) Less(i, j int) bool {
	return b[i].to < b[j].to
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	b := make(bridges, M)
	for i := range b {
		fmt.Scan(&b[i].from, &b[i].to)
	}

	sort.Sort(b)
	last := 0
	ans := 0
	for i := range b {
		if last < b[i].from {
			last = b[i].to - 1
			ans++
		}
	}

	fmt.Println(ans)
}
