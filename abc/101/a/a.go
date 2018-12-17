// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sun Dec 16 19:33:27 2018
//
package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)

	number := 0
	for _, c := range S {
		if c == '+' {
			number++
		} else {
			number--
		}
	}

	fmt.Println(number)
}
