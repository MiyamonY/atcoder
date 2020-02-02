// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Jan 18 17:10:29 2020
//
package main

import "fmt"

var N, M int

func main() {
	fmt.Scan(&N, &M)

	acs := make([]bool, N)
	was := make([]int, N)
	for i := 0; i < M; i++ {
		var p int
		var s string
		fmt.Scan(&p, &s)
		if s == "WA" {
			if !acs[p-1] {
				was[p-1]++
			}

		} else {
			acs[p-1] = true
		}
	}

	numAC := 0
	for i := range acs {
		if acs[i] {
			numAC++
		}
	}
	numWA := 0
	for i := range was {
		if acs[i] {
			numWA += was[i]
		}
	}
	fmt.Println(numAC, numWA)
}
