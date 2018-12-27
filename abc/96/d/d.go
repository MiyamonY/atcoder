///
// File:  d.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 13:42:57 2018
//
package main

import (
	"fmt"
)

func isPrime(n int) bool {
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func primes() []int {
	ret := make([]int, 0)
	for i := 2; i <= 55555; i++ {
		if isPrime(i) {
			ret = append(ret, i)
		}
	}
	return ret
}

func main() {
	var N int
	fmt.Scan(&N)

	ps := primes()
	ans := make([]int, 0)
	for i := range ps {
		if ps[i]%5 == 1 {
			ans = append(ans, ps[i])
			if len(ans) == N {
				break
			}
		}
	}

	for i := range ans {
		if i != 0 {
			fmt.Print(" ")
		}
		fmt.Print(ans[i])
	}
	fmt.Println()
}
