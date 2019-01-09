// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Wed Jan  9 09:29:22 2019
//
package main

import (
	"fmt"
)

const MOD = 1e9 + 7

func main() {
	var N int
	fmt.Scan(&N)
	var s1, s2 string
	fmt.Scan(&s1)
	fmt.Scan(&s2)

	ans := 1
	prevUp := true
	for i := 0; i < len(s1); i++ {
		if i == 0 {
			if s1[i] == s2[i] {
				ans *= 3
				prevUp = true
			} else {
				ans *= 6
				i++
				prevUp = false
			}
		} else {
			if s1[i] == s2[i] {
				if prevUp {
					ans *= 2
				} else {
					ans *= 1
				}
				prevUp = true
			} else {
				if prevUp {
					ans *= 2
				} else {
					ans *= 3
				}
				i++
				prevUp = false
			}
		}
		ans %= MOD
	}
	fmt.Println(ans)
}
