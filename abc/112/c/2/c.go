// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Dec 19 22:03:02 2018
//
package main

import "fmt"

type point struct {
	x, y, h int
}

func abs(a int) int {
	if a < 0 {
		return -a
	}

	return a
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]point, N)
	for i := range arr {
		fmt.Scan(&arr[i].x, &arr[i].y, &arr[i].h)
	}

	for x := 0; x <= 100; x++ {
		for y := 0; y <= 100; y++ {
			valid := true
			h := 0
			for _, p := range arr {
				if p.h > 0 {
					h = p.h + abs(p.x-x) + abs(p.y-y)
				}
			}

			for _, p := range arr {
				if max(h-abs(x-p.x)-abs(y-p.y), 0) != p.h {
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
