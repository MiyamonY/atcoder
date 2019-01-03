// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Thu Jan  3 15:43:19 2019
//
package main

import "fmt"

type train struct {
	c, s, f int
}

func main() {
	var N int
	fmt.Scan(&N)

	trains := make([]train, N-1)
	for i := range trains {
		fmt.Scan(&trains[i].c, &trains[i].s, &trains[i].f)
	}

	for i := 0; i < N; i++ {
		time := 0
		for j := i; j < len(trains); j++ {
			t := trains[j]
			if time > t.s {
				n := ((time - t.s) + t.f - 1) / t.f
				time = t.s + n*t.f
			} else {
				time = t.s
			}
			time += t.c
		}

		fmt.Println(time)
	}
}
