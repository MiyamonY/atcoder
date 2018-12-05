// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Wed Dec  5 01:13:12 2018
//
package main

import "fmt"

var k int

func main() {
	fmt.Scan(&k)
	half := k / 2
	if k%2 == 0 {
		fmt.Println(half * half)
	} else {
		fmt.Println(half * (half + 1))
	}
}
