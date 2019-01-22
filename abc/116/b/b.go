// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Tue Jan 22 16:26:07 2019
//
package main

import "fmt"

func main() {
	var s int
	fmt.Scan(&s)

	arr := []int{s}
	for s != 1 {
		if s%2 == 0 {
			s /= 2
		} else {
			s = 3*s + 1
		}
		arr = append(arr, s)
	}

	arr = append(arr, []int{4, 2, 1}...)
	for i := range arr {
		for j := i + 1; j < len(arr); j++ {
			if arr[i] == arr[j] {
				fmt.Println(j + 1)
				return
			}
		}
	}

}
