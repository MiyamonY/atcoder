// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Jan 29 11:30:46 2019
//
package main

import (
	"fmt"
	"sort"
)

type pair struct {
	x, y int
}

type blues []pair

func (ps blues) Len() int {
	return len(ps)
}

func (ps blues) Swap(i, j int) {
	ps[i], ps[j] = ps[j], ps[i]
}

func (ps blues) Less(i, j int) bool {
	return ps[i].x < ps[j].x
}

type reds []pair

func (ps reds) Len() int {
	return len(ps)
}

func (ps reds) Swap(i, j int) {
	ps[i], ps[j] = ps[j], ps[i]
}

func (ps reds) Less(i, j int) bool {
	return ps[i].y > ps[j].y
}

func main() {
	var N int
	fmt.Scan(&N)

	reds := make(reds, N)
	for i := range reds {
		fmt.Scan(&reds[i].x, &reds[i].y)
	}

	blues := make(blues, N)
	for i := range blues {
		fmt.Scan(&blues[i].x, &blues[i].y)
	}
	sort.Sort(reds)
	sort.Sort(blues)

	count := 0
	used := make([]bool, len(reds))
	for _, b := range blues {
		for i, r := range reds {
			if !used[i] && r.x < b.x && r.y < b.y {
				used[i] = true
				count++
				break
			}
		}
	}

	fmt.Println(count)
}
