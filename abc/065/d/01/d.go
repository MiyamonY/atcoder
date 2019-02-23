// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan 16 16:43:18 2019
//
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type point struct {
	index int
	x, y  int
}

func (p point) xLess(q point) bool {
	if p.x == q.x {
		return p.y < q.y
	}
	return p.x < q.x
}

func (p point) yLess(q point) bool {
	if p.y == q.y {
		return p.x < q.y
	}

	return p.y < q.y
}

type xpoints []point

func (xp xpoints) Len() int {
	return len(xp)
}

func (xp xpoints) Less(i, j int) bool {
	return xp[i].xLess(xp[j])
}

func (xp xpoints) Swap(i, j int) {
	xp[i], xp[j] = xp[j], xp[i]
}

type ypoints []point

func (yp ypoints) Len() int {
	return len(yp)
}

func (yp ypoints) Less(i, j int) bool {
	return yp[i].yLess(yp[j])
}

func (yp ypoints) Swap(i, j int) {
	yp[i], yp[j] = yp[j], yp[i]
}

type route struct {
	to, weight int
}

type item struct {
	r     route
	index int
}

type pQ []*item

func (pq pQ) Len() int {
	return len(pq)
}

func (pq pQ) Less(i, j int) bool {
	return pq[i].r.weight < pq[j].r.weight
}

func (pq pQ) Swap(i, j int) {
	pq[i].r, pq[j].r = pq[j].r, pq[i].r
	pq[i].index, pq[j].index = pq[j].index, pq[i].index
}

func (pq *pQ) Push(p interface{}) {
	x := p.(*item)
	x.index = len(*pq)
	*pq = append(*pq, x)
}

func (pq *pQ) Pop() interface{} {
	old := *pq
	n := len(old)
	i := old[n-1]
	*pq = old[0 : n-1]
	return i
}

func prim(g [][]route) int {
	visited := make([]bool, len(g))

	visited[0] = true
	pq := make(pQ, len(g[0]))
	for i := range g[0] {
		r := g[0][i]
		pq[i] = &item{r: r}
	}
	heap.Init(&pq)

	ans := 0
	for len(pq) > 0 {
		p := heap.Pop(&pq).(*item)
		if !visited[p.r.to] {
			ans += p.r.weight
			visited[p.r.to] = true
			for i := range g[p.r.to] {
				heap.Push(&pq, &item{r: g[p.r.to][i]})
			}
		}
	}

	return ans
}

func main() {
	var N int
	fmt.Scan(&N)

	xps := make(xpoints, N)
	yps := make(ypoints, N)
	for i := range xps {
		fmt.Scan(&xps[i].x, &xps[i].y)
		xps[i].index = i
		yps[i].index, yps[i].x, yps[i].y = xps[i].index, xps[i].x, xps[i].y
	}
	sort.Sort(xps)
	sort.Sort(yps)

	g := make([][]route, N)
	for i := range xps {
		if i+1 < len(xps) {
			d := xps[i+1].x - xps[i].x
			a, b := xps[i].index, xps[i+1].index
			g[a] = append(g[a], route{to: b, weight: d})
			g[b] = append(g[b], route{to: a, weight: d})
		}
		if i+1 < len(yps) {
			d := yps[i+1].y - yps[i].y
			a, b := yps[i].index, yps[i+1].index
			g[a] = append(g[a], route{to: b, weight: d})
			g[b] = append(g[b], route{to: a, weight: d})
		}
	}

	fmt.Println(prim(g))
}
