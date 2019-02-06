// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Tue Feb  5 12:14:53 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type tuple struct {
	t, x, y int
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

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	sc.Split(bufio.ScanWords)
	N := nextInt()

	ts := make([]tuple, N)
	for i := range ts {
		ts[i].t, ts[i].x, ts[i].y = nextInt(), nextInt(), nextInt()
	}
	for i := range ts {
		var t, d int
		if i == 0 {
			t = ts[0].t
			d = abs(ts[0].x) + abs(ts[i].y)

		} else {
			t = ts[i].t - ts[i-1].t
			d = abs(ts[i].x-ts[i-1].x) + abs(ts[i].y-ts[i-1].y)
		}

		if t < d || (d-t)%2 != 0 {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
