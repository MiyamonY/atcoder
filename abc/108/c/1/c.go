// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Dec  5 22:15:09 2018
//
package main

import "fmt"

var n, k int

func main() {
	fmt.Scan(&n, &k)

	if k%2 == 1 {
		m := n / k
		fmt.Println(m * m * m)
	} else {
		m := n / k
		m2 := n/(k/2) - m

		fmt.Println(m*m*m + m2*m2*m2)
	}
}
