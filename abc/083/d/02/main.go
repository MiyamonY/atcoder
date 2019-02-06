// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Wed Feb  6 12:22:28 2019
//
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var S string
	fmt.Scan(&S)

	ans := len(S)
	for i := range S {
		if i+1 < len(S) {
			if S[i] != S[i+1] {
				ans = min(ans, max(i+1, len(S)-i-1))
			}
		}
	}

	fmt.Println(ans)
}
