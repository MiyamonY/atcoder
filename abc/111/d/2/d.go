// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Dec 19 22:42:09 2018
//
package main

import "fmt"

type point struct {
	x, y int
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func checkPossible(points []point) bool {
	oe := abs(points[0].x+points[0].y) % 2
	for _, p := range points {
		if oe != abs(p.x+p.y)%2 {
			return false
		}
	}
	return true
}

func pow2(n int) int {
	if n == 0 {
		return 1
	}
	return 2 * pow2(n-1)
}

func main() {
	var N int
	fmt.Scan(&N)
	points := make([]point, N)
	for i := range points {
		fmt.Scan(&points[i].x, &points[i].y)
	}

	if !checkPossible(points) {
		fmt.Println(-1)
		return
	}
	arms := []int{}
	if abs(points[0].x+points[0].y)%2 == 0 {
		arms = append(arms, 1)
	}

	for i := 0; i < 32; i++ {
		arms = append(arms, pow2(i))
	}

	fmt.Println(len(arms))
	for i, arm := range arms {
		if i != 0 {
			fmt.Print(" ")
		}
		fmt.Print(arm)
	}
	fmt.Println()

	for _, p := range points {
		u := p.x + p.y
		v := p.x - p.y
		modes := ""
		for i := len(arms) - 1; i >= 0; i-- {
			l := arms[i]
			if u >= 0 && v >= 0 {
				u -= l
				v -= l
				modes += "R"
			} else if u >= 0 && v <= 0 {
				u -= l
				v += l
				modes += "U"
			} else if u <= 0 && v >= 0 {
				u += l
				v -= l
				modes += "D"
			} else {
				u += l
				v += l
				modes += "L"
			}
		}
		for i := len(modes) - 1; i >= 0; i-- {
			fmt.Print(string(modes[i]))
		}
		fmt.Println()
	}
}
