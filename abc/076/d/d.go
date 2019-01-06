// Package main provides
//
// File:  d.go
// Author: ymiyamoto
//
// Created on Mon Jan  7 00:42:17 2019
//
package main

import (
	"fmt"
	"math"
)

func scanSlice(slice []int) {
	for i := range slice {
		fmt.Scan(&slice[i])
	}
}

func main() {
	var N int
	fmt.Scan(&N)

	ts := make([]int, N)
	scanSlice(ts)
	ts = append([]int{0}, ts...)
	for i := range ts {
		ts[i] *= 2
	}

	vs := make([]int, N)
	scanSlice(vs)

	for i := range ts {
		if i+1 < len(ts) {
			ts[i+1] += ts[i]
		}
	}

	totalTime := ts[len(ts)-1]
	vecs := make([][]float64, len(vs))
	for i := range vecs {
		vecs[i] = make([]float64, totalTime+1)
		for j := range vecs[i] {
			vecs[i][j] = math.MaxFloat64
		}
	}

	for i := range vs {
		lowerTime := ts[i]
		upperTime := ts[i+1]
		for j := lowerTime; j <= upperTime; j++ {
			vecs[i][j] = float64(vs[i])
		}

		if i+1 < len(vs) && vs[i] < vs[i+1] {
			vec := float64(vs[i])
			for j := upperTime; j < len(vecs[i]); j++ {
				vecs[i][j] = vec
				vec += 0.5
			}
		}
		if i > 0 && vs[i] < vs[i-1] {
			vec := float64(vs[i])
			for j := lowerTime; j >= 0; j-- {
				vecs[i][j] = vec
				vec += 0.5
			}
		}
	}

	v := make([]float64, totalTime+1)
	speed := 0.0
	for i := range v {
		v[i] = speed
		speed += 0.5
	}
	vecs = append(vecs, v)

	v = make([]float64, totalTime+1)
	speed = 0.0
	for j := len(v) - 1; j >= 0; j-- {
		v[j] = speed
		speed += 0.5
	}
	vecs = append(vecs, v)

	var vec []float64
	for t := range vecs[0] {
		min := math.MaxFloat64
		for i := range vecs {
			if vecs[i][t] < min {
				min = vecs[i][t]
			}
		}
		vec = append(vec, min)
	}

	ans := 0.0
	for i := range vec {
		if i+1 < len(vec) {
			ans += (vec[i] + vec[i+1]) / 4.0
		}
	}

	fmt.Println(ans)
}
