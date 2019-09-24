// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Mon Sep 23 03:17:40 2019
//
package main

import (
	"container/heap"
	"fmt"
)

type Heap []int

func (h Heap) Len() int {
	return len(h)
}

func (h Heap) Less(i, j int) bool {
	return h[i] > h[j]
}

func (h Heap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *Heap) Push(x interface{}) {
	y, _ := x.(int)
	*h = append(*h, y)
}

func (h *Heap) Pop() interface{} {
	n := len(*h)
	x := (*h)[n-1]
	*h = (*h)[:n-1]
	return x
}

func main() {
	var n, m int
	fmt.Scan(&n, &m)

	h := make(Heap, n)
	for i := range h {
		fmt.Scan(&h[i])
	}

	heap.Init(&h)
	for i := 0; i < m; i++ {
		x := heap.Pop(&h)
		y, _ := x.(int)
		heap.Push(&h, y/2)
	}
	ans := 0
	for h.Len() > 0 {
		x, _ := heap.Pop(&h).(int)
		ans += x
	}

	fmt.Println(ans)
}
