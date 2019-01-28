// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Dec 28 16:01:15 2018
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
		var A, B int
		fmt.Scan(&A, &B)
		if B < A {
			A, B = B, A
		}

		if A == B {
			fmt.Println(2*A - 2)
		} else if A+1 == B {
			fmt.Println(2*A - 2)
		} else {
			c := int(math.Sqrt(float64(A * B)))
			if A*B == c*c {
				c--
			}
			if c*(c+1) >= A*B {
				fmt.Println(2*c - 2)
			} else {
				fmt.Println(2*c - 1)
			}
		}
	}
}
