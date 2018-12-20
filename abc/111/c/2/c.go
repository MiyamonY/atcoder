// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Wed Dec 19 22:26:27 2018
//
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxVal(m map[int]int) (int, int) {
	max := 0
	key := 0
	for k, v := range m {
		if max < v {
			key = k
			max = v
		}
	}

	return key, max
}

func main() {
	var n int
	fmt.Scan(&n)

	arr := make([]int, n)
	m1, m2 := map[int]int{}, map[int]int{}
	for i := range arr {
		fmt.Scan(&arr[i])
		if i%2 == 0 {
			m1[arr[i]]++
		} else {
			m2[arr[i]]++
		}
	}

	key1, max1 := maxVal(m1)
	key2, max2 := maxVal(m2)
	if key1 != key2 {
		fmt.Println(n - max1 - max2)
	} else {
		m1[key1] = 0
		_, max3 := maxVal(m1)
		m2[key2] = 0
		_, max4 := maxVal(m2)
		fmt.Println(min(n-max1-max4, n-max2-max3))
	}
}
