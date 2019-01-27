// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 14:25:28 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	sum := 0
	left := 0
	ans := 0
	for i := range arr {
		for sum^arr[i] != sum+arr[i] {
			sum -= arr[left]
			left++
		}
		sum += arr[i]
		ans += i - left + 1
	}
	fmt.Println(ans)
}
