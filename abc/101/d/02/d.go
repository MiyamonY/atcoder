// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 24 16:20:46 2019
//
package main

import (
	"fmt"
	"sort"
	"strconv"
)

func digitSum(n int) int {
	s := strconv.Itoa(n)

	sum := 0
	for i := range s {
		sum += int(s[i] - '0')
	}
	return sum
}

func unique(slice []int) []int {
	ret := []int{}
	for _, s := range slice {
		hasSame := false
		for _, r := range ret {
			if r == s {
				hasSame = true
			}
		}
		if !hasSame {
			ret = append(ret, s)
		}
	}

	return ret
}

func pow10(n int) int {
	if n == 0 {
		return 1
	}
	return 10 * pow10(n-1)
}

func main() {
	var K int
	fmt.Scan(&K)
	candidates := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}

	for i := 1; i <= 15; i++ {
		for j := 1; j <= 150; j++ {
			n := pow10(i)
			if j*(n-1) <= 1e15 {
				candidates = append(candidates, j*n+n-1)
			}
		}
	}

	candidates = unique(candidates)
	sort.Ints(candidates)

	anss := make([]int, 0)
	for i, n := range candidates {
		valid := true
		for j := i + 1; j < len(candidates); j++ {
			m := candidates[j]
			if valid && digitSum(m)*n > digitSum(n)*m {
				valid = false
			}
		}
		if valid {
			anss = append(anss, n)
		}
	}

	for i := 0; i < K; i++ {
		fmt.Println(anss[i])
	}
}
