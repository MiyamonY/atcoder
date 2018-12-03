// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec  2 21:35:07 2018
//
package main

import "fmt"

var n int

func num(n int, arr []int) int {
	ret := 0
	for i := range arr {
		if n <= arr[i] {
			ret++
		}
	}
	return ret
}

func main() {
	fmt.Scan(&n)

	d := map[int]int{}

	for i := 1; i <= n; i++ {
		val := i
		for j := 2; j <= i; j++ {
			for val%j == 0 {
				val /= j
				d[j]++
			}
		}
	}
	var arr []int
	for _, v := range d {
		arr = append(arr, v+1)
	}

	ans := num(75, arr) + num(25, arr)*(num(3, arr)-1) + num(15, arr)*(num(5, arr)-1) + num(5, arr)*(num(5, arr)-1)*(num(3, arr)-2)/2
	fmt.Println(ans)
}
