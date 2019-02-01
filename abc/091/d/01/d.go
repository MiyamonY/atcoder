// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sat Dec 29 14:26:15 2018
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

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
	var N int
	fmt.Scan(&N)

	sc.Split(bufio.ScanWords)
	arra := make([]int, N)
	for i := 0; i < N; i++ {
		arra[i] = nextInt()
	}

	arrb := make([]int, N)
	for i := 0; i < N; i++ {
		arrb[i] = nextInt()
	}

	ans := 0
	arr := make([]int, N)
	for i := 0; i < 30; i++ {
		num := 0
		for j, b := range arrb {
			arr[j] = b % (1 << uint8(i+1))
		}
		sort.Ints(arr)

		for _, a := range arra {
			a %= (1 << uint8(i+1))
			x := sort.Search(len(arr), func(k int) bool {
				return a+arr[k] >= (1 << uint8(i))
			})
			y := sort.Search(len(arr), func(k int) bool {
				return a+arr[k] >= 2*(1<<uint8(i))
			})

			z := sort.Search(len(arr), func(k int) bool {
				return a+arr[k] >= 3*(1<<uint8(i))
			})
			w := sort.Search(len(arr), func(k int) bool {
				return a+arr[k] >= 4*(1<<uint8(i))
			})
			num += y - x + w - z
		}

		if num%2 == 1 {
			ans |= 1 << uint8(i)
		}
	}
	fmt.Printf("%d\n", ans)
}
