// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Jan 19 09:21:18 2019
//
package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strconv"
)

type intHeap []int

func (h intHeap) sum() int {
	s := 0
	for i := range h {
		s += h[i]
	}
	return s
}

func (h intHeap) Len() int {
	return len(h)
}

func (h intHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h intHeap) Less(i, j int) bool {
	return h[i] < h[j]
}

func (h *intHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *intHeap) Pop() interface{} {
	old, n := *h, len(*h)
	v := old[n-1]
	*h = (*h)[:n-1]
	return v
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

	arr := make([]int, 3*N)
	for i := range arr {
		arr[i] = nextInt()
	}

	leftHeap := make(intHeap, N)
	copy(leftHeap, arr[:N])

	sumLefts := make([]int, N+1)
	heap.Init(&leftHeap)
	for k := N - 1; k < 2*N; k++ {
		if k == N-1 {
			sumLefts[0] = leftHeap.sum()
			continue
		}
		heap.Push(&leftHeap, arr[k])
		n := heap.Pop(&leftHeap).(int)
		sumLefts[k-N+1] = sumLefts[k-N] + arr[k] - n
	}

	rightHeap := make(intHeap, N)
	for i := range rightHeap {
		rightHeap[i] = -arr[2*N+i]
	}

	sumRights := make([]int, N+1)
	heap.Init(&rightHeap)
	for k := 2 * N; k >= N; k-- {
		if k == 2*N {
			sumRights[len(sumRights)-1] = -rightHeap.sum()
			continue
		}
		heap.Push(&rightHeap, -arr[k])
		n := -heap.Pop(&rightHeap).(int)
		sumRights[k-N] = sumRights[k-N+1] + arr[k] - n
	}

	max := -1 << 60
	for i := range sumRights {
		d := sumLefts[i] - sumRights[i]
		if max < d {
			max = d
		}
	}

	fmt.Println(max)
}
