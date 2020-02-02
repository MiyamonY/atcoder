// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 15:08:28 2020
//
package main

import "fmt"

const MOD = 1e9 + 7

var H, N int
var A []int

func main() {
	fmt.Scan(&H, &N)
	A = make([]int, N)

	sum := 0
	for i := range A {
		fmt.Scan(&A[i])
		sum += A[i]
	}

	if H <= sum {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
