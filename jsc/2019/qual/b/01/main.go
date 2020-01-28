// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Aug 24 21:22:23 2019
//
package main

import "fmt"

const MOD = 1e9 + 7

func main() {
	var n, k int64
	fmt.Scan(&n, &k)

	arr := make([]int, n)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	var count int64
	for i := range arr {
		for j := i + 1; j < len(arr); j++ {
			if arr[i] > arr[j] {
				count++
			}
		}
	}
	ans := (count * k) % MOD

	count = 0
	for i := range arr {
		for j := range arr {
			if arr[j] < arr[i] {
				count++
			}
		}
	}
	fmt.Println((ans + (count*((k*(k-1)/2)%MOD))%MOD) % MOD)
}
