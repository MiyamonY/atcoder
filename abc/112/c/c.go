// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 14:07:23 2018
//
package main

import (
	"fmt"
)

var n int

type pos struct {
	x, y, h int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func calcHeight(x, y, h int, p pos) int {
	return max(h-abs(p.x-x)-abs(p.y-y), 0)
}

func main() {
	fmt.Scan(&n)

	poss := make([]pos, n)
	for i := range poss {
		fmt.Scan(&poss[i].x, &poss[i].y, &poss[i].h)
	}

	for x := 0; x <= 100; x++ {
		for y := 0; y <= 100; y++ {
			var h int
			for _, p := range poss {
				if p.h > 0 {
					h = abs(p.x-x) + abs(p.y-y) + p.h
					break
				}
			}

			valid := true
			for _, p := range poss {
				if calcHeight(x, y, h, p) != p.h {
					valid = false
				}
			}

			if valid {
				fmt.Println(x, y, h)
				return
			}
		}
	}
}
