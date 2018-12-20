// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Fri Dec 21 00:20:49 2018
//
package main

import "fmt"

func main() {
	var S, T string
	fmt.Scan(&S, &T)

	m := map[uint8]uint8{}
	for i := range S {
		if c, ok := m[S[i]]; ok {
			if c != T[i] {
				fmt.Println("No")
				return
			}
		}
		m[S[i]] = T[i]
	}

	m = map[uint8]uint8{}
	for i := range T {
		if c, ok := m[T[i]]; ok {
			if c != S[i] {
				fmt.Println("No")
				return
			}
		}
		m[T[i]] = S[i]
	}
	fmt.Println("Yes")
}
