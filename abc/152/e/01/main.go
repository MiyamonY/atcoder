// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Sun Feb  2 11:20:26 2020
//
package main

import (
	"fmt"
	"math/big"
)

const MOD = 1e9 + 7

var N int
var A []*big.Int

func solve() int64 {
	mod := big.NewInt(MOD)

	lcm := big.NewInt(A[0].Int64())
	for _, a := range A {
		g := big.NewInt(0).GCD(nil, nil, lcm, a)
		lcm.Mul(lcm, a)
		lcm.Div(lcm, g)
	}

	ans := big.NewInt(0)
	for _, a := range A {
		x := big.NewInt(0).Div(lcm, a)
		ans.Add(ans, x)
	}
	return ans.Mod(ans, mod).Int64()
}

func main() {
	fmt.Scan(&N)
	A = make([]*big.Int, N)
	for i := range A {
		var a int64
		fmt.Scan(&a)
		A[i] = big.NewInt(a)
	}

	fmt.Println(solve())
}
