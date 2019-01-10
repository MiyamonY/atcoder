// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 22:18:19 2019
//
package main

import "fmt"

func printSlice(slice []int) {
	for i := range slice {
		if i != 0 {
			fmt.Print(" ")
		}
		fmt.Print(slice[i])
	}
	fmt.Println()
}

func main() {
	var K int
	fmt.Scan(&K)
	n := 50

	arr := make([]int, n)
	for i := range arr {
		arr[i] = i
	}

	q, r := K/50, K%50
	for i := range arr {
		arr[i] += q
	}
	for i := 0; i < r; i++ {
		arr[49-i]++
	}

	fmt.Println(n)
	printSlice(arr)
}
