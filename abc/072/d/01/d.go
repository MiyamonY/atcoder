// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 00:36:47 2019
//
package main

import "fmt"

func scanSlice(slice []int) {
	for i := range slice {
		fmt.Scan(&slice[i])
	}
}

func main() {
	var N int
	fmt.Scan(&N)
	arr := make([]int, N)
	scanSlice(arr)

	ans := 0
	for i := range arr {
		if i < len(arr)-1 {
			if arr[i] == i+1 {
				arr[i], arr[i+1] = arr[i+1], arr[i]
				ans++
			}
		} else {
			if arr[i] == i+1 {
				arr[i], arr[i-1] = arr[i-1], arr[i]
				ans++
			}
		}
	}
	fmt.Println(ans)
}
