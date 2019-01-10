// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Fri Jan 11 02:25:48 2019
//
package main

import "fmt"

func main() {
	var S string
	fmt.Scan(&S)
	for i := 1; i < len(S); i++ {
		s := S[:len(S)-i]
		if s[:len(s)/2] == s[len(s)/2:] {
			fmt.Println(len(s))
			return
		}
	}
}
