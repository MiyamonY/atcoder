// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 22:43:45 2018
//
package main

import "fmt"

func main() {
	var N int
	_, _ = fmt.Scan(&N)
	m := map[string]int{}
	for i := 0; i < N; i++ {
		var s string
		_, _ = fmt.Scan(&s)
		m[s]++
	}

	if len(m) == 3 {
		fmt.Println("Three")
	} else {
		fmt.Println("Four")
	}
}
