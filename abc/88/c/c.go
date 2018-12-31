// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Mon Dec 31 13:26:37 2018
//
package main

import "fmt"

func main() {
	arr := make([][]int, 3)
	for i := range arr {
		arr[i] = make([]int, 3)
		for j := range arr[i] {
			fmt.Scan(&arr[i][j])
		}
	}

	t1 := arr[0][1] - arr[0][0]
	t2 := arr[0][2] - arr[0][1]
	s1 := arr[1][0] - arr[0][0]
	s2 := arr[2][0] - arr[1][0]
	for i := range arr {
		if arr[i][1]-arr[i][0] != t1 || arr[i][2]-arr[i][1] != t2 {
			fmt.Println("No")
			return
		}
		if arr[1][i]-arr[0][i] != s1 || arr[2][i]-arr[1][i] != s2 {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
