// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 15:36:25 2019
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	m := map[int]int{}
	for i := 0; i < N; i++ {
		var a int
		fmt.Scan(&a)
		m[a]++
	}

	arr := make([]int, 0)
	for _, v := range m {
		arr = append(arr, v)
	}
	sort.Ints(arr)

	ans := 0
	for i := 0; i < len(m)-K; i++ {
		ans += arr[i]
	}
	fmt.Println(ans)
}
