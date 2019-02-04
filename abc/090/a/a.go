// Package main provides
//
// File:  a.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 18:01:19 2018
//
package main

import "fmt"

func main() {
	arr := make([]string, 3)
	for i := range arr {
		fmt.Scan(&arr[i])
	}

	for i := range arr {
		fmt.Print(string(arr[i][i]))
	}

	fmt.Println()
}
