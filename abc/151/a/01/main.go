// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jan 18 17:02:43 2020
//
package main

import "fmt"

var c string

func main() {
	fmt.Scan(&c)
	fmt.Println(string(c[0] + 1))
}
