// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 15:41:00 2018
//
package main

import "fmt"

var n int

func allSame(a int) bool {
	for _, i := range []int{111, 222, 333, 444, 555, 666, 777, 888, 999} {
		if a == i {
			return true
		}
	}
	return false
}

func main() {
	fmt.Scan(&n)
	for i := n; i <= 999; i++ {
		if allSame(i) {
			fmt.Println(i)
			return
		}
	}
}
