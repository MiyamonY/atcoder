// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Tue Jan  8 23:48:31 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var N int
	fmt.Scan(&N)
	for _, c := range strconv.Itoa(N) {
		if c == '9' {
			fmt.Println("Yes")
			return
		}
	}
	fmt.Println("No")
}
