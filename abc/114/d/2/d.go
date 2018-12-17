// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Dec 17 15:23:05 2018
//
package main

import "fmt"

func nums(slice []int, f func(n int) bool) int {
	count := 0
	for _, s := range slice {
		if f(s) {
			count++
		}
	}
	return count
}

func gTE(n int) func(n int) bool {
	return func(m int) bool { return m >= n-1 }
}

func main() {
	var N int
	fmt.Scan(&N)

	m := map[int]int{}
	for i := 2; i <= N; i++ {
		n := i
		for j := 2; j <= i; j++ {
			for n%j == 0 {
				m[j]++
				n /= j
			}
		}
	}

	arr := make([]int, 0)
	for _, v := range m {
		arr = append(arr, v)
	}

	ans := nums(arr, gTE(75)) + nums(arr, gTE(25))*(nums(arr, gTE(3))-1) + nums(arr, gTE(15))*(nums(arr, gTE(5))-1) + nums(arr, gTE(5))*(nums(arr, gTE(5))-1)*(nums(arr, gTE(3))-2)/2
	fmt.Println(ans)
}
