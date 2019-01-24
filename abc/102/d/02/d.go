// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Jan 24 14:25:11 2019
//
package main

import "fmt"

func scanSlice(slice []int) {
	for i := range slice {
		fmt.Scan(&slice[i])
	}
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func max(args ...int) int {
	m := 0
	for _, a := range args {
		if m < a {
			m = a
		}
	}
	return m
}

func min(args ...int) int {
	m := 1 << 60
	for _, a := range args {
		if a < m {
			m = a
		}
	}

	return m
}

func equlibium(slice []int) (ps []int, qs []int) {
	index := 1
	for k := 2; k <= len(slice)-2; k++ {
		p := slice[index-1]
		q := slice[k-1] - slice[index-1]
		diff := abs(p - q)
		for index+1 < k {
			a := slice[index]
			b := slice[k-1] - slice[index]
			if abs(a-b) > diff {
				break
			}
			diff = abs(a - b)
			index++
			p = slice[index-1]
			q = slice[k-1] - slice[index-1]
		}
		ps = append(ps, p)
		qs = append(qs, q)
	}

	return ps, qs
}

func main() {
	var N int
	fmt.Scan(&N)

	slice := make([]int, N)
	scanSlice(slice)

	acc := make([]int, N)
	for i := range slice {
		if i == 0 {
			acc[i] = slice[0]
		} else {
			acc[i] = slice[i] + acc[i-1]
		}
	}
	ps, qs := equlibium(acc)

	revAcc := make([]int, N)
	for i := range revAcc {
		if i == 0 {
			revAcc[i] = slice[len(slice)-1]
		} else {
			revAcc[i] = slice[len(slice)-1-i] + revAcc[i-1]
		}
	}
	rs, ss := equlibium(revAcc)

	ans := 1 << 60
	for i := range ps {
		p, q, r, s := ps[i], qs[i], rs[len(ps)-1-i], ss[len(ps)-1-i]
		diff := max(p, q, r, s) - min(p, q, r, s)
		if diff < ans {
			ans = diff
		}
	}

	fmt.Println(ans)
}
