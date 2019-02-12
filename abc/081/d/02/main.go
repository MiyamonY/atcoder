// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Feb  9 15:10:24 2019
//
package main

import "fmt"

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	min, max := 1<<60, -1<<60
	minI, maxI := 0, 0
	for i := range arr {
		fmt.Scan(&arr[i])
		if arr[i] < min {
			min = arr[i]
			minI = i
		}
		if max < arr[i] {
			max = arr[i]
			maxI = i
		}
	}

	if min >= 0 {
		fmt.Println(N - 1)
		for i := 1; i+1 <= N; i++ {
			fmt.Println(i, i+1)
		}
	} else if max <= 0 {
		fmt.Println(N - 1)
		for i := N; i-1 >= 1; i-- {
			fmt.Println(i, i-1)
		}
	} else if abs(min) < abs(max) {
		fmt.Println(2*N - 1)
		for i := 0; i < N; i++ {
			fmt.Println(maxI+1, i+1)
		}
		for i := 0; i < N-1; i++ {
			fmt.Println(i+1, i+2)
		}
	} else {
		fmt.Println(2*N - 1)
		for i := 0; i < N; i++ {
			fmt.Println(minI+1, i+1)
		}
		for i := N - 1; i >= 1; i-- {
			fmt.Println(i+1, i)
		}
	}
}
