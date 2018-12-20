// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Fri Dec 21 01:19:23 2018
//
package main

import "fmt"

const MOD = 1e9 + 7

func fact(n int) int {
	if n == 0 {
		return 1
	}
	return (n * fact(n-1)) % MOD
}

func pow(a, b int) int {
	if b == 0 {
		return 1
	}
	if b%2 == 0 {
		return pow((a*a)%MOD, b/2) % MOD
	}
	return (a * pow(a, b-1)) % MOD
}

func comb(a, b int) int {
	x := fact(a)
	y := pow(fact(b), MOD-2)
	z := pow(fact(a-b), MOD-2)
	return (((x * y) % MOD) * z) % MOD
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	m := map[int]int{}
	for i := 2; i*i <= M; i++ {
		for M%i == 0 {
			m[i]++
			M /= i
		}
	}

	if M != 1 {
		m[M]++
	}

	ans := 1
	for _, k := range m {
		ans *= comb(k+N-1, k)
		ans %= MOD
	}
	fmt.Println(ans)
}
