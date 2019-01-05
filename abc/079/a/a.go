// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 17:01:58 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var N int
	fmt.Scan(&N)

	n := strconv.Itoa(N)
	if (n[0] == n[1] && n[1] == n[2]) || (n[1] == n[2] && n[2] == n[3]) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
