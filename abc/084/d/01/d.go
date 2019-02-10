// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 16:51:59 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
)

var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}

func main() {
	var Q int
	fmt.Scan(&Q)

	table := make([]bool, 1e5+1)
	for i := range table {
		table[i] = true
	}
	table[1] = false

	for i := 2; i*i <= 1e5; i++ {
		if table[i] {
			for j := i + i; j <= 1e5; j += i {
				table[j] = false
			}
		}
	}

	likes := make([]int, 1e5+1)
	for i := range likes {
		if table[i] && table[(i+1)/2] {
			likes[i]++
		}
	}

	for i := range likes {
		if i+1 < len(likes) {
			likes[i+1] += likes[i]
		}
	}

	for i := 0; i < Q; i++ {
		var l, r int
		fmt.Sscan(nextLine(), &l, &r)
		fmt.Println(likes[r] - likes[l-1])
	}
}
