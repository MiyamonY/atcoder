// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Dec  5 21:48:31 2018
//
package main

import "fmt"

var x1, x2, y1, y2 int

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	fmt.Scan(&x1, &y1, &x2, &y2)
	dx := x2 - x1
	dy := y2 - y1

	x3 := x2 - dy
	y3 := y2 + dx

	fmt.Println(x3, y3, x3-dx, y3-dy)
}
