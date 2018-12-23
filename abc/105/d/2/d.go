// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec 23 23:41:00 2018
//
package main

import "fmt"

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	arr := make([]int, N)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	for i := range arr {
		if i+1 < len(arr) {
			arr[i+1] += arr[i]
		}
		arr[i] %= M
	}

	m := map[int]int{}
	for _, a := range arr {
		m[a]++
	}

	ans := m[0]
	for _, v := range m {
		ans += v * (v - 1) / 2
	}

	fmt.Println(ans)
}
