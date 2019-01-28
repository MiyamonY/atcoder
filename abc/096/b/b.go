///
// File:  b.go
// Author: ymiyamoto
//
// Created on Thu Dec 27 13:13:05 2018
//
package main

import (
  "fmt"
)

func max(arr... int)int{
	ret := 0
	for i := range arr{
		if ret < arr[i] {
			ret = arr[i]
		}
	}
	return ret
}

func pow2(n int)int{
	if n == 0{
		return 1
	}
	return 2 * pow2(n-1)
}

func main(){
	var A, B, C int
	var K int
	fmt.Scan(&A, &B, &C)
	fmt.Scan(&K)

	fmt.Println(A+B+C-max(A,B,C) + pow2(K)*max(A,B,C))
}
