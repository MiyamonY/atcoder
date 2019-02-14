// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sun Jan  6 00:21:47 2019
//
package main

import "fmt"

func main() {
	var s1, s2 string
	fmt.Scan(&s1, &s2)

	for i := range s1 {
		if s1[i] != s2[len(s2)-1-i] {
			fmt.Println("NO")
			return
		}
	}
	fmt.Println("YES")
}
