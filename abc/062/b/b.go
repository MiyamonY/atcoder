// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 15:51:53 2019
//
package main

import (
	"fmt"
	"strings"
)

func main() {
	var H, W int
	fmt.Scan(&H, &W)

	fmt.Println(strings.Repeat("#", W+2))
	for i := 0; i < H; i++ {
		var s string
		fmt.Scan(&s)
		fmt.Println("#" + s + "#")
	}
	fmt.Println(strings.Repeat("#", W+2))
}
