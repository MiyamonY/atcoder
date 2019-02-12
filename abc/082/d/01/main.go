// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Jan  4 16:20:38 2019
//
package main

import (
	"fmt"
)

const maxLen = 8000

func canReach(fs []int, at int, startDirectionPuls bool) bool {
	dp := make([][]bool, len(fs)+1)
	for i := range dp {
		dp[i] = make([]bool, 2*maxLen+1)
	}
	if startDirectionPuls {
		dp[0][maxLen+fs[0]] = true
		fs = fs[1:]
	} else {
		dp[0][maxLen+0] = true
	}

	for i := range fs {
		for f := range dp[i] {
			if dp[i][f] {
				dp[i+1][f-fs[i]] = true
				dp[i+1][f+fs[i]] = true
			}
		}
	}

	return dp[len(fs)][maxLen+at]
}

func splitCommand(s string) (xs []int, ys []int) {
	arms, move := []int{}, 0
	for _, c := range s {
		if c == 'T' {
			arms = append(arms, move)
			move = 0
		} else {
			move++
		}
	}
	arms = append(arms, move)

	for i := range arms {
		if i%2 == 0 {
			xs = append(xs, arms[i])
		} else {
			ys = append(ys, arms[i])
		}
	}
	return xs, ys
}

func solve(s string, x, y int) bool {
	xs, ys := splitCommand(s)
	return canReach(xs, x, true) && canReach(ys, y, false)
}

func main() {
	var s string
	fmt.Scan(&s)
	var x, y int
	fmt.Scan(&x, &y)

	if solve(s, x, y) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
