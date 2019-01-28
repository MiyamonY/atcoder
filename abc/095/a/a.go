// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 21:10:07 2018
//
package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)

	ans := 700
	for i := range S {
		if S[i] == 'o' {
			ans += 100
		}
	}

	fmt.Println(ans)
}
