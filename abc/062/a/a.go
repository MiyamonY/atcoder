// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 15:26:11 2019
//
package main

import "fmt"

func main() {
	m := map[int]int{1: 1, 3: 1, 5: 1, 7: 1, 8: 1, 10: 1, 12: 1, 4: 2, 6: 2, 9: 2, 11: 2, 2: 3}

	var x, y int
	fmt.Scan(&x, &y)
	if m[x] == m[y] {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
