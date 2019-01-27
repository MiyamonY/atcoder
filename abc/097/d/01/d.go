// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Dec 26 21:40:31 2018
//
package main

import (
	"fmt"
	"sort"
)

type node struct {
	index  int
	parent int
}

type unionFind struct {
	nodes []node
	rank  []int
}

func new(n int) unionFind {
	u := unionFind{}
	u.nodes = make([]node, n)
	u.rank = make([]int, n)
	for i := range u.nodes {
		u.nodes[i].index = i
		u.nodes[i].parent = i
	}
	return u
}

func (u unionFind) top(a int) int {
	p := u.nodes[a].parent
	for p != u.nodes[p].parent {
		p = u.nodes[p].parent
	}

	return p
}

func (u unionFind) union(a, b int) {
	ap := u.top(a)
	bp := u.top(b)
	if u.rank[ap] == u.rank[bp] {
		u.nodes[ap].parent = bp
		u.rank[bp]++
	} else if u.rank[ap] < u.rank[bp] {
		u.nodes[ap].parent = bp
	} else {
		u.nodes[bp].parent = ap
	}
}

func (u unionFind) find(a int) int {
	return u.top(a)
}

func (u unionFind) sets() map[int][]int {
	m := map[int][]int{}
	for i := range u.nodes {
		if _, ok := m[u.top(i)]; ok {
			m[u.top(i)] = append(m[u.top(i)], i)
		} else {
			m[u.top(i)] = []int{i}
		}
	}

	return m
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	ps := make([]int, N)
	for i := range ps {
		fmt.Scan(&ps[i])
	}

	u := new(N)
	for i := 0; i < M; i++ {
		var x, y int
		fmt.Scan(&x, &y)
		x--
		y--
		u.union(x, y)
	}

	ans := 0
	for _, set := range u.sets() {
		var vals, indexs []int
		for _, i := range set {
			vals = append(vals, ps[i]-1)
			indexs = append(indexs, i)
		}
		sort.Ints(vals)
		sort.Ints(indexs)
		i := 0
		j := 0
		for i < len(vals) && j < len(indexs) {
			if vals[i] == indexs[j] {
				ans++
				i++
				j++
			} else if vals[i] < indexs[j] {
				i++
			} else {
				j++
			}
		}
	}

	fmt.Println(ans)
}
