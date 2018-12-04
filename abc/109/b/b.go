// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Dec  5 00:30:32 2018
//
package main

import "fmt"

var n int

func main() {
	fmt.Scan(&n)
	arr := make([]string, n)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	for i := range arr {
		for j := i + 1; j < len(arr); j++ {
			if arr[i] == arr[j] {
				fmt.Println("No")
				return
			}
		}

		if i+1 < len(arr) {
			if arr[i][len(arr[i])-1] != arr[i+1][0] {
				fmt.Println("No")
				return
			}
		}
	}

	fmt.Println("Yes")
}
