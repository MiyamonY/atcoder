// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 21:44:08 2019
//
package main

import (
	"fmt"
)

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func main() {
	var s string
	fmt.Scan(&s)

	minVal := len(s)
	for i := range s {
		if i+1 < len(s) && s[i] != s[i+1] {
			n := max(i+1, len(s)-i-1)
			if n < minVal {
				minVal = n
			}
		}
	}

	fmt.Println(minVal)
}
