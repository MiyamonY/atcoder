// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Wed Feb  6 11:57:01 2019
//
package main

import "fmt"

var primes []bool

func main() {
	primes := make([]bool, 1e5+1)
	for i := range primes {
		primes[i] = true
	}
	primes[0], primes[1] = false, false

	for i := 2; i <= 1e5; i++ {
		if primes[i] {
			for j := i + i; j <= 1e5; j += i {
				primes[j] = false
			}
		}
	}

	arr := make([]int, 1e5+1)
	for i := range arr {
		if primes[i] && primes[(i+1)/2] {
			arr[i]++
		}
	}

	for i := range arr {
		if i > 0 {
			arr[i] += arr[i-1]
		}
	}

	var Q int
	fmt.Scan(&Q)
	for i := 0; i < Q; i++ {
		var l, r int
		fmt.Scan(&l, &r)
		fmt.Println(arr[r] - arr[l-1])
	}
}
