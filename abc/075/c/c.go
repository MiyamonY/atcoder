// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 02:12:06 2019
//
package main

import "fmt"

type route struct {
	to    int
	valid bool
}

type node struct {
	routes []route
}

type nodes []node

func dfs(n int, graph nodes, visited []bool) {
	visited[n] = true

	rs := graph[n].routes
	for _, r := range rs {
		if r.valid && !visited[r.to] {
			dfs(r.to, graph, visited)
		}
	}
}

func checkNodes(graph nodes) bool {
	visited := make([]bool, len(graph))
	dfs(0, graph, visited)

	for i := range visited {
		if !visited[i] {
			return false
		}
	}
	return true
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	graph := make(nodes, N)
	for i := 0; i < M; i++ {
		var a, b int
		fmt.Scan(&a, &b)
		graph[a-1].routes = append(graph[a-1].routes, route{to: b - 1, valid: true})
		graph[b-1].routes = append(graph[b-1].routes, route{to: a - 1, valid: true})
	}

	ans := 0
	for i := range graph {
		for j := range graph[i].routes {
			graph[i].routes[j].valid = false
			if !checkNodes(graph) {
				ans++
			}
			graph[i].routes[j].valid = true
		}
	}

	fmt.Println(ans)
}
