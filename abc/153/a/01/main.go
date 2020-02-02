// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 15:07:45 2020
//
package main

import "fmt"

const MOD = 1e9 + 7

var H, A int

func main() {
	fmt.Scan(&H, &A)
	fmt.Println((H + A - 1) / A)
}
