// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Dec  2 05:03:29 2018
//
package main

import "fmt"

var s, t string

func main() {
	fmt.Scan(&s, &t)

	for n := 0; n < 2; n++ {
		arr := make([]byte, 26)

		for i := range s {
			index := t[i] - 'a'
			if arr[index] == 0 {
				arr[index] = s[i]
			} else {
				if arr[index] != s[i] {
					fmt.Println("No")
					return
				}
			}
		}
		s, t = t, s
	}
	fmt.Println("Yes")
}
