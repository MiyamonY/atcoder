// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan 17 15:56:11 2019
//
package main

import (
	"fmt"
	"sort"
)

var H, W int

func max(slice ...int) int {
	sort.Ints(slice)
	return slice[len(slice)-1]
}

func min(slice ...int) int {
	sort.Ints(slice)
	return slice[0]
}

func minDiffArea(a, b, c int) int {
	return max(a, b, c) - min(a, b, c)
}

func splitH() int {
	arr := make([]int, H-1)
	for h := range arr {
		a := (h + 1) * W
		b := (H - (h + 1)) / 2 * W
		c := H*W - a - b
		arr[h] = minDiffArea(a, b, c)
	}
	return min(arr...)
}

func splitV() int {
	arr := make([]int, W-1)
	for w := range arr {
		a := (w + 1) * H
		b := (W - (w + 1)) / 2 * H
		c := H*W - a - b
		arr[w] = minDiffArea(a, b, c)
	}
	return min(arr...)
}

func splitHV() int {
	arr := make([]int, H-1)
	for h := range arr {
		a := (H - h - 1) * W
		b := (h + 1) * (W / 2)
		c := H*W - a - b
		arr[h] = minDiffArea(a, b, c)
	}
	return min(arr...)
}

func splitVH() int {
	arr := make([]int, W-1)
	for w := range arr {
		a := (W - w - 1) * H
		b := (w + 1) * (H / 2)
		c := H*W - a - b
		arr[w] = minDiffArea(a, b, c)
	}

	return min(arr...)
}

func main() {
	fmt.Scan(&H, &W)

	fmt.Println(min(splitH(), splitHV(), splitV(), splitVH()))
}
