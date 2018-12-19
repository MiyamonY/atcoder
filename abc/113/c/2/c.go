// Package main provides
//
// File:  c.go
// Author: ymiyamoto
//
// Created on Mon Dec 17 17:56:12 2018
//
package main

import (
	"fmt"
	"sort"
)

type city struct {
	pref, year, no int
}

type cities []city

func (c cities) Len() int {
	return len(c)
}

func (c cities) Swap(i, j int) {
	c[i], c[j] = c[j], c[i]
}

func (c cities) Less(i, j int) bool {
	if c[i].pref == c[j].pref {
		return c[i].year < c[j].year
	}
	return c[i].pref < c[j].pref
}

func main() {
	var N, M int
	fmt.Scan(&N, &M)

	cs := make(cities, M)
	for i := range cs {
		fmt.Scan(&cs[i].pref, &cs[i].year)
	}

	cs2 := make(cities, len(cs))
	copy(cs2, cs)
	sort.Sort(cs)

	count := 1
	for i := range cs {
		if i > 0 && cs[i-1].pref != cs[i].pref {
			count = 1
		}
		cs[i].no = count
		count++
	}

	for _, c := range cs2 {
		lower := sort.Search(len(cs), func(i int) bool {
			return cs[i].pref >= c.pref
		})
		upper := sort.Search(len(cs), func(i int) bool {
			return cs[i].pref >= c.pref+1
		})
		prefs := cs[lower:upper]

		index := sort.Search(len(prefs), func(i int) bool {
			return prefs[i].year >= c.year
		})
		fmt.Printf("%06d%06d\n", prefs[index].pref, prefs[index].no)
	}
}
