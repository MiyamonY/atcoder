// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Tue Dec 11 00:35:24 2018
//
package main

import (
	"fmt"
	"sort"
)

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func scanSlice(slice []int) {
	for i := 0; i < len(slice); i++ {
		fmt.Scan(&slice[i])
	}
}

func accumulateSlice(slice []int) {
	length := len(slice)
	for i := 0; i < length; i++ {
		if i+1 < length {
			slice[i+1] += slice[i]
		}
	}
}

func leftSums(arr []int, left, mid int) (int, int) {
	return arr[left-1], arr[mid-1] - arr[left-1]
}

func rightSums(arr []int, mid, right int) (int, int) {
	return arr[right-1] - arr[mid-1], arr[len(arr)-1] - arr[right-1]
}

func findRight(arr []int, mid int) int {
	diff := 1000000000
	right := mid + 1
	for i := mid + 1; i < len(arr)-1; i++ {
		lower, upper := rightSums(arr, mid, i)
		if abs(lower-upper) < diff {
			diff = abs(lower - upper)
			right = i
		}
	}
	return right
}

func diffSums(sums ...int) int {
	sort.Ints(sums)
	return sums[len(sums)-1] - sums[0]
}

func main() {
	var N int

	fmt.Scan(&N)
	arr := make([]int, N)
	scanSlice(arr)
	accumulateSlice(arr)

	minVal := 100000000000
	left := 1
	right := findRight(arr, 2)

	for mid := 2; mid < len(arr)-1; mid++ {
		p, q := leftSums(arr, left, mid)
		for diffLeft := abs(p - q); left < mid; left++ {
			newP, newQ := leftSums(arr, left+1, mid)
			if abs(newP-newQ) > diffLeft {
				break
			}
			diffLeft = abs(newP - newQ)
		}

		r, s := rightSums(arr, mid, right)
		for diffRight := abs(r - s); right < len(arr)-1; right++ {
			newR, newS := rightSums(arr, mid, right+1)
			if abs(newR-newS) > diffRight {
				break
			}
			diffRight = abs(newR - newS)
		}

		P, Q := leftSums(arr, left, mid)
		R, S := rightSums(arr, mid, right)
		val := diffSums(P, Q, R, S)

		if val < minVal {
			minVal = val
		}
	}
	fmt.Println(minVal)
}
