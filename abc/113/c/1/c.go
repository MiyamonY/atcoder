// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Sat Dec  1 02:30:33 2018
//
package main

import (
	"fmt"
	"sort"
)

type city struct {
	p int
	y int
}

var n, m int
var cityYears [100001][]int
var cities []city

func main() {
	fmt.Scan(&n, &m)

	cities = make([]city, m)
	for i := range cityYears {
		cityYears[i] = make([]int, 0)
	}

	for i := range cities {
		fmt.Scan(&cities[i].p, &cities[i].y)
		cityYears[cities[i].p] = append(cityYears[cities[i].p], cities[i].y)
	}

	for i := range cityYears {
		sort.Ints(cityYears[i])
	}

	for i := range cities {
		p, y := cities[i].p, cities[i].y
		index := sort.Search(len(cityYears[p]), func(i int) bool { return cityYears[p][i] >= y })
		fmt.Printf("%06d%06d\n", p, index+1)
	}
}
