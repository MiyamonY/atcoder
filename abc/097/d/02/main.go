// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Jan 27 15:07:05 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type unionFind struct {
	roots []int
}

func new(n int) unionFind {
	uf := unionFind{}
	uf.roots = make([]int, n+1)

	for i := range uf.roots {
		uf.roots[i] = i
	}

	return uf
}

func (uf unionFind) root(n int) int {
	parent := uf.roots[n]

	if parent == n {
		return n
	}
	root := uf.root(parent)
	uf.roots[n] = root
	return root
}

func (uf unionFind) union(n, m int) {
	rootN := uf.root(n)
	rootM := uf.root(m)
	if rootN < rootM {
		uf.roots[rootN] = rootM
	} else {
		uf.roots[rootM] = rootN
	}
}

var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func main() {
	sc.Split(bufio.ScanWords)
	N := nextInt()
	M := nextInt()

	arr := make([]int, N)
	for i := range arr {
		arr[i] = nextInt()
	}

	uf := new(N)
	for i := 0; i < M; i++ {
		n := nextInt()
		m := nextInt()
		uf.union(n, m)
	}

	m1 := map[int][]int{}
	m2 := map[int]map[int]bool{}
	for i := range arr {
		m1[uf.root(arr[i])] = append(m1[uf.root(arr[i])], i+1)
		if m2[uf.root(arr[i])] == nil {
			m2[uf.root(arr[i])] = map[int]bool{}
		}
		m2[uf.root(arr[i])][arr[i]] = true
	}

	ans := 0
	for k, v := range m1 {
		for i := range v {
			if _, ok := m2[k][v[i]]; ok {
				ans++
			}
		}
	}
	fmt.Println(ans)
}
