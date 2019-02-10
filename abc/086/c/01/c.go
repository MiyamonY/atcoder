// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 22:49:46 2019
//
package main

import "fmt"

type point struct {
	t, x, y int
}

func (p point) distance() int {
	return abs(p.x) + abs(p.y)
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)

	ps := make([]point, N)
	for i := range ps {
		fmt.Scan(&ps[i].t, &ps[i].x, &ps[i].y)
	}

	for i := range ps {
		p := ps[i]
		if p.distance() > p.t {
			fmt.Println("No")
			return
		}

		if i > 0 {
			pred := ps[i-1]
			dist := abs(p.x-pred.x) + abs(p.y-pred.y)
			if dist > p.t-pred.t {
				fmt.Println("No")
				return
			}
		}

		if (p.t%2 == 0 && p.distance()%2 == 1) || (p.t%2 == 1 && p.distance()%2 == 0) {
			fmt.Println("No")
			return
		}
	}

	fmt.Println("Yes")
}
