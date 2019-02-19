// Package main provides
//
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Jan 10 09:45:26 2019
//
package main

import (
	"fmt"
	"strconv"
)

func main() {
	var s string
	fmt.Scan(&s)
	fmt.Println(string(s[0]) + strconv.Itoa(len(s)-2) + string(s[len(s)-1]))
}
