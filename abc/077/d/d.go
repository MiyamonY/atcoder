// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Jan  6 15:04:41 2019
//
package main

import (
	"container/heap"
	"fmt"
)

type route struct {
	to, weight int
}

type node struct {
	routes []route
	index  int
	weight int
}

type priQ []*node

func (pq priQ) Len() int {
	return len(pq)
}

func (pq priQ) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq priQ) Less(i, j int) bool {
	return pq[i].weight < pq[j].weight
}

func (pq *priQ) Push(x interface{}) {
	n := len(*pq)
	r := x.(*node)
	r.index = n
	*pq = append(*pq, r)
}

func (pq *priQ) Pop() interface{} {
	old := *pq
	n := len(*pq)
	r := old[n-1]
	*pq = old[:n-1]
	return r
}

func (pq *priQ) update(n *node) {
	heap.Fix(pq, n.index)
}

func dikstra(start int, end int, graph []node) int {
	graph[start].weight = 0

	pq := priQ{}
	heap.Init(&pq)
	heap.Push(&pq, &graph[start])
	pq.update(&graph[start])

	for pq.Len() > 0 {
		n := heap.Pop(&pq).(*node)
		for _, r := range n.routes {
			w := n.weight + r.weight
			if w < graph[r.to].weight {
				graph[r.to].weight = w

				heap.Push(&pq, &graph[r.to])
			}
		}
	}

	return graph[end].weight
}

func main() {
	var K int
	fmt.Scan(&K)

	graph := make([]node, K)
	for i := range graph {
		if i+1 < len(graph) {
			graph[i].routes = append(graph[i].routes, route{to: i + 1, weight: 1})
		} else {
			graph[i].routes = append(graph[i].routes, route{to: 0, weight: 1})
		}
		graph[i].routes = append(graph[i].routes, route{to: (i * 10) % K, weight: 0})
		graph[i].index = i
		graph[i].weight = 1 << 60
	}

	fmt.Println(dikstra(1, 0, graph) + 1)
}
