// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 09:47:06 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	m := map[int]int{}
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		if a%4 == 0 {
			m[4]++
		} else if a%2 == 0 {
			m[2]++
		}
	}

	if m[4] >= N/2 || m[4] >= (N-(m[2]/2)*2)/2 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
