// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Feb  4 12:20:15 2019
//
package main

import "fmt"

func main() {
	var N int
	fmt.Scan(&N)

	m := map[byte]int{}
	for i := 0; i < N; i++ {
		var s string
		fmt.Scan(&s)
		m[s[0]]++
	}

	ans := 0
	for _, s := range []string{"MAR", "MAC", "MAH", "MRC", "MRH", "MCH", "ARC", "ARH", "ACH", "RCH"} {
		count := 1
		for i := range s {
			count *= m[s[i]]
		}
		ans += count
	}
	fmt.Println(ans)
}
