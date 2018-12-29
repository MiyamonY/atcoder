// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec 29 04:55:53 2018
//
package main

import (
	"fmt"
	"sort"
)

type point struct {
	x, y int
}

func (p point) lessThan(q point) bool {
	return p.x < q.x && p.y < q.y
}

type points []point

// (p points) ...
func (p points) Less(i, j int) bool {
	if p[i].x == p[j].x {
		return p[i].y < p[j].y
	}

	return p[i].x < p[j].x
}

// (p points)Len ...
func (p points) Len() int {
	return len(p)
}

// (p points)Swap ...
func (p points) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}
func main() {
	var N int

	fmt.Scan(&N)
	reds := make(points, N)
	for i := range reds {
		fmt.Scan(&reds[i].x, &reds[i].y)
	}
	sort.Sort(reds)

	blues := make(points, N)
	for i := range blues {
		fmt.Scan(&blues[i].x, &blues[i].y)
	}
	sort.Sort(blues)

	ans := 0
	paired := make([]bool, N)
	for _, b := range blues {
		max := -1
		pair := -1
		for j, r := range reds {
			if r.x < b.x && r.y < b.y && max < r.y && !paired[j] {
				max = r.y
				pair = j
			}
		}
		if pair != -1 {
			paired[pair] = true
			ans++
		}
	}

	fmt.Printf("%d\n", ans)
}
