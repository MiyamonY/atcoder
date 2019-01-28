// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Jan 28 14:21:15 2019
//
package main

import (
	"fmt"
	"math"
)

func main() {
	var Q int
	fmt.Scan(&Q)
	for i := 0; i < Q; i++ {
		var a, b int
		fmt.Scan(&a, &b)
		if a > b {
			a, b = b, a
		}

		if a == b {
			fmt.Println(2 * (a - 1))
		} else if a+1 == b {
			fmt.Println(2 * (a - 1))
		} else {
			c := int(math.Sqrt(float64(a * b)))
			if a*b == c*c {
				c--
			}
			if c*(c+1) >= a*b {
				fmt.Println(2 * (c - 1))
			} else {
				fmt.Println(2*c - 1)
			}
		}
	}
}
