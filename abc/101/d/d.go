// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec 16 19:39:22 2018
//
package main

import (
	"fmt"
	"sort"
)

type int64s []int64

func (slice int64s) Less(i, j int) bool {
	return slice[i] < slice[j]
}

func (slice int64s) Len() int {
	return len(slice)
}

func (slice int64s) Swap(i, j int) {
	slice[i], slice[j] = slice[j], slice[i]
}

func pow10(n int) int64 {
	if n == 0 {
		return 1
	}
	return 10 * pow10(n-1)
}

func digitSums(n int64) int64 {
	var sum int64

	rem := n
	for rem > 0 {
		sum += rem % 10
		rem /= 10
	}

	return sum
}

func unique(slice int64s) int64s {
	m := map[int64]bool{}
	for _, val := range slice {
		m[val] = true
	}

	uniqued := make(int64s, 0)
	for k := range m {
		uniqued = append(uniqued, k)
	}
	return uniqued
}

func main() {
	var K int
	fmt.Scan(&K)

	candidates := make(int64s, 0)
	for p := int64(1); p <= 150; p++ {
		for n := 0; n <= 15; n++ {
			candidates = append(candidates, p*pow10(n)-1)
		}
	}
	candidates = unique(candidates[1:])
	sort.Sort(candidates)

	numbers := make([]float64, 0)
	for _, candidate := range candidates {
		numbers = append(numbers, float64(candidate)/float64(digitSums(candidate)))
	}

	sunukes := make(int64s, 0)
	for i, num := range numbers {
		valid := true
		for j := i + 1; j < len(numbers); j++ {
			if num > numbers[j] {
				valid = false
			}
		}
		if valid {
			sunukes = append(sunukes, candidates[i])
		}
	}

	for i := 0; i < K; i++ {
		fmt.Println(sunukes[i])
	}
}
