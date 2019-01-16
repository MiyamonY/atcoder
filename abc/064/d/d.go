// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 10:46:05 2019
//
package main

import (
	"fmt"
	"strings"
)

func main() {
	var N int
	fmt.Scan(&N)
	var S string
	fmt.Scan(&S)

	leftNum := 0
	for i := range S {
		if S[len(S)-1-i] == ')' {
			leftNum++
		} else {
			if leftNum > 0 {
				leftNum--
			}
		}
	}

	s := strings.Repeat("(", leftNum) + S
	rightNum := 0
	for i := range s {
		if s[i] == '(' {
			rightNum++
		} else {
			if rightNum > 0 {
				rightNum--
			}

		}
	}

	fmt.Println(s + strings.Repeat(")", rightNum))
}
