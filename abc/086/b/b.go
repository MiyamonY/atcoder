// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Wed Jan  2 22:46:48 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	v, _ := strconv.Atoi(strconv.Itoa(a) + strconv.Itoa(b))

	for i := 1; i*i <= v; i++ {
		if i*i == v {
			fmt.Println("Yes")
			return
		}
	}
	fmt.Println("No")
}
