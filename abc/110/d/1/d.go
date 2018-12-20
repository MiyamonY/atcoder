// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Sun Dec  2 05:20:19 2018
//
package main

import "fmt"

const mod = 1e9 + 7

var n, m int64

func primeFactorization(x int64) map[int64]int64 {
	val := x
	ret := map[int64]int64{}

	for i := int64(2); i*i <= x; i++ {
		for val%i == 0 {
			val /= i
			ret[i]++
		}
	}

	if val > 1 {
		ret[val]++
	}

	return ret
}

func combination(n, k, p int64) int64 {
	var fact func(n int64) int64
	fact = func(n int64) int64 {
		if n == 0 {
			return 1
		}
		return n * fact(n-1) % p
	}

	var power func(n, m, p int64) int64
	power = func(n, m, p int64) int64 {
		if m == 0 {
			return 1
		}

		n2 := n * n % p
		if m%2 == 0 {
			return power(n2, m/2, p) % p
		}
		return n * power(n2, (m-1)/2, p) % p
	}

	x := fact(n)
	y := fact(k)
	z := fact(n - k)
	return x * (power(y, p-2, p) * power(z, p-2, p) % p) % p
}

func main() {
	fmt.Scan(&n, &m)

	var ans int64 = 1
	for _, v := range primeFactorization(m) {
		ans *= combination(v+n-1, v, mod)
		ans %= mod
	}

	fmt.Println(ans)
}
