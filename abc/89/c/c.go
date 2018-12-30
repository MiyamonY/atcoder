// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sun Dec 30 22:53:11 2018
//
package main

import "fmt"

func bits(b int) int {
	ret := 0
	for i := byte(0); i < 32; i++ {
		if b&(1<<byte(i)) != 0 {
			ret++
		}
	}
	return ret
}

func main() {
	var N int
	_, _ = fmt.Scan(&N)

	m := map[byte]int{}
	for i := 0; i < N; i++ {
		var s string
		_, _ = fmt.Scan(&s)
		m[s[0]]++
	}

	ans := 0
	for i := 0; i < 1<<5; i++ {
		if bits(i) != 3 {
			continue
		}

		t := []byte{'M', 'A', 'R', 'C', 'H'}
		num := 1
		for j := 0; j < 5; j++ {
			if i&(1<<uint8(j)) != 0 {
				num *= m[t[j]]
			}
		}
		ans += num
	}

	fmt.Println(ans)
}
