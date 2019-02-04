// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  3 21:03:15 2019
//
package main

import (
	"fmt"
	"sort"
)

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	slice := make([]int, M)
	for i := range slice {
		fmt.Scan(&slice[i])
	}
	sort.Ints(slice)

	if M <= N {
		fmt.Println(0)
		return
	}

	tmp := make([]int, M)
	for i := range slice {
		if i > 0 {
			tmp[i] = slice[i] - slice[i-1]
		}
	}

	sort.Ints(tmp)
	ans := 0
	for i := 0; i < M-N+1; i++ {
		ans += tmp[i]
	}

	fmt.Println(ans)
}
