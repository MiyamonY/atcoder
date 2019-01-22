// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Tue Jan 22 16:40:32 2019
//
package main

import (
	"fmt"
	"sort"
)

type pair struct {
	t, d int
}

type pairs []pair

func (ps pairs) Len() int {
	return len(ps)
}

func (ps pairs) Swap(i, j int) {
	ps[i], ps[j] = ps[j], ps[i]
}

func (ps pairs) Less(i, j int) bool {
	return ps[i].d > ps[j].d
}

func main() {
	var N, K int
	fmt.Scan(&N, &K)

	s := make(pairs, N)
	for i := range s {
		fmt.Scan(&s[i].t, &s[i].d)
	}
	sort.Sort(s)

	dSum := 0
	kinds := map[int]int{}
	for i := 0; i < K; i++ {
		dSum += s[i].d
		kinds[s[i].t]++
	}
	k := len(kinds)

	max := dSum + k*k
	minIndex := K - 1
	for i := K; i < len(s); i++ {
		t, d := s[i].t, s[i].d
		if _, ok := kinds[t]; ok {
			continue
		}
		kinds[t]++

		for kinds[s[minIndex].t] <= 1 {
			minIndex--
			if minIndex == -1 {
				fmt.Println(max)
				return
			}
		}
		minT, minD := s[minIndex].t, s[minIndex].d
		kinds[minT]--

		minIndex--

		dSum += -minD + d
		k++
		if max < dSum+k*k {
			max = dSum + k*k
		}
	}
	fmt.Println(max)
}
