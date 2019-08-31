// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sat Aug 24 21:16:12 2019
//
package main

import "fmt"

func main() {
	var m, d int
	fmt.Scan(&m)
	fmt.Scan(&d)
	count := 0

	for i := 1; i <= m; i++ {
		for j := 1; j <= d; j++ {
			d10 := j / 10
			d1 := j % 10
			if d10 >= 2 && d1 >= 2 {
				if i == d10*d1 {
					count++
				}
			}
		}
	}
	fmt.Println(count)
}
