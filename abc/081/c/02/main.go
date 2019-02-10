// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Feb  9 15:06:41 2019
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

	nums := make([]int, 0)
	for _, v := range m {
		nums = append(nums, v)
	}
	sort.Ints(nums)

	ans := 0
	for i := 0; i+K < len(nums); i++ {
		ans += nums[i]
	}

	fmt.Println(ans)
}
