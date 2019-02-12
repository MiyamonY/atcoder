// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan  5 16:43:46 2019
//
package main

import (
	"fmt"
	"sort"
)

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

type pair struct {
	begin, end int
}

type pairs []pair

func (p pairs) Len() int {
	return len(p)
}

func (p pairs) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p pairs) Less(i, j int) bool {
	return p[i].begin < p[j].begin
}

func main() {
	var N, C int
	fmt.Scan(&N, &C)

	m := map[int]pairs{}
	for i := 0; i < N; i++ {
		var p pair
		var c int
		fmt.Scan(&p.begin, &p.end, &c)
		if _, ok := m[c]; ok {
			m[c] = append(m[c], p)
		} else {
			m[c] = pairs{p}
		}
	}

	for c, v := range m {
		arr := pairs{}
		sort.Sort(v)
		for i := 0; i < len(v); i++ {
			if len(arr) == 0 || arr[len(arr)-1].end != v[i].begin {
				arr = append(arr, v[i])
			} else {
				arr[len(arr)-1].end = v[i].end
			}
		}
		m[c] = arr
	}

	arr := make([]int, 10e5+1)
	for _, v := range m {
		for _, p := range v {
			arr[p.begin-1]++
			arr[p.end]--
		}
	}

	for i := range arr {
		if i+1 < len(arr) {
			arr[i+1] += arr[i]
		}
	}

	fmt.Println(max(arr...))
}
