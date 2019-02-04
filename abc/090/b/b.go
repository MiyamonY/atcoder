// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 18:16:12 2018
//
package main

import (
	"fmt"
	"strconv"
)

func isPalidrome(s string) bool {
	for i := range s {
		if s[i] != s[len(s)-1-i] {
			return false
		}
	}
	return true
}

func main() {
	var A, B int
	fmt.Scan(&A, &B)

	ans := 0
	for i := A; i <= B; i++ {
		if isPalidrome(strconv.Itoa(i)) {
			ans++
		}
	}

	fmt.Println(ans)
}
