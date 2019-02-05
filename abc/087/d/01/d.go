// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 22:09:57 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
)

const invalidNum = 1 << 60

type route struct {
	to, weight int
}

type node struct {
	routes []route
}

var nodes []node
var nums []int
var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}

func dfs(top int, num int) bool {
	if nums[top] != invalidNum {
		return nums[top] == num
	}

	nums[top] = num
	for _, r := range nodes[top].routes {
		if !dfs(r.to, num+r.weight) {
			return false
		}
	}

	return true
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	nodes = make([]node, N+1)
	for i := 0; i < M; i++ {
		s := nextLine()
		var l, r, d int
		fmt.Sscan(s, &l, &r, &d)
		nodes[l].routes = append(nodes[l].routes, route{to: r, weight: d})
		nodes[r].routes = append(nodes[r].routes, route{to: l, weight: -d})
	}

	nums = make([]int, N+1)
	for i := range nums {
		nums[i] = invalidNum
	}

	for i := range nodes {
		if nums[i] == invalidNum && !dfs(i, 0) {
			fmt.Println("No")
			return
		}
	}

	fmt.Println("Yes")
}
