// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 17:02:49 2018
//
package main

import "fmt"

type pos struct {
	x, y int
}

var n int

func pow(n int) int {
	if n == 0 {
		return 1
	}
	return 2 * pow(n-1)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	fmt.Scan(&n)

	poss := make([]pos, n)
	for i := range poss {
		fmt.Scan(&poss[i].x, &poss[i].y)
	}

	parity := (abs(poss[0].x) + abs(poss[0].y)) % 2
	for _, p := range poss {
		if (abs(p.x)+abs(p.y))%2 != parity {
			fmt.Println(-1)
			return
		}
	}

	var dist []int
	if parity == 0 {
		dist = []int{1}
	} else {
		dist = []int{}
	}

	for i := 0; i < 33; i++ {
		dist = append(dist, pow(i))
	}

	fmt.Println(len(dist))

	for i, d := range dist {
		fmt.Print(d)
		if i < len(dist)-1 {
			fmt.Print(" ")
		} else {
			fmt.Println()
		}
	}

	for _, p := range poss {
		x, y := p.x, p.y
		route := ""
		for i := len(dist) - 1; i >= 0; i-- {
			d := dist[i]
			if abs(x) > abs(y) {
				if x > 0 {
					route += "R"
					x -= d
				} else {
					route += "L"
					x += d
				}
			} else {
				if y > 0 {
					route += "U"
					y -= d
				} else {
					route += "D"
					y += d
				}
			}
		}

		for i := len(route) - 1; i >= 0; i-- {
			fmt.Print(string(route[i]))
		}
		fmt.Println()
	}
}
