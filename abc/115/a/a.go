// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sat Dec  8 21:11:58 2018
//
package main

import "fmt"

func main() {
	var D int
	fmt.Scan(&D)
	var t string
	if D == 25 {
		t = "Christmas"
	} else if D == 24 {
		t = "Christmas Eve"
	} else if D == 23 {
		t = "Christmas Eve Eve"
	} else if D == 22 {
		t = "Christmas Eve Eve Eve"
	}

	fmt.Println(t)
}
