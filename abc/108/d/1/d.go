// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Dec  6 00:31:35 2018
//
package main

import "fmt"

var l int

type line struct {
	from   int
	weight int
}

func pow(n int) int {
	ret := 1
	for i := 0; i < n; i++ {
		ret *= 2
	}
	return ret
}

func main() {
	fmt.Scan(&l)

	var n int
	for i := 1; pow(i)-1 < l; i++ {
		n = i
	}

	maxWeights := pow(n) - 1
	var lines []line
	for i := n - 1; i >= 0; i-- {
		weight := pow(i) - 1
		newWeigts := (maxWeights + 1) + weight
		if newWeigts < l {
			lines = append(lines, line{from: i, weight: maxWeights + 1})
			maxWeights = newWeigts
		}
	}

	fmt.Println(n+1, 2*n+len(lines))
	for i := 0; i < n; i++ {
		fmt.Println(i+1, i+1+1, pow(i))
		fmt.Println(i+1, i+1+1, 0)
	}

	for _, line := range lines {
		fmt.Println(line.from+1, n+1, line.weight)
	}
}
