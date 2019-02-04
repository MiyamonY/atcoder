// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Feb  4 14:11:22 2019
//
package main

import "fmt"

func main() {

	var m [3][3]int
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			fmt.Scan(&m[i][j])
		}
	}

	b1, b2, b3 := m[0][0], m[0][1], m[0][2]
	a21, a22, a23 := m[1][0]-b1, m[1][1]-b2, m[1][2]-b3
	a31, a32, a33 := m[2][0]-b1, m[2][1]-b2, m[2][2]-b3

	if a21 != a22 || a22 != a23 || a31 != a32 || a32 != a33 {
		fmt.Println("No")
		return
	}

	fmt.Println("Yes")
}
