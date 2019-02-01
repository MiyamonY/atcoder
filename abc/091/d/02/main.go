// Package main provides
//
// File:  main.go
// Author: ymiyamoto
//
// Created on Fri Feb  1 12:29:03 2019
//
package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var rdr = bufio.NewReaderSize(os.Stdin, 1000000)

func readLine() string {
	buf := make([]byte, 0, 1000000)
	for {
		l, p, e := rdr.ReadLine()
		if e != nil {
			panic(e)
		}
		buf = append(buf, l...)
		if !p {
			break
		}
	}
	return string(buf)
}

func nextInt(sc *bufio.Scanner) int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func main() {
	sc := bufio.NewScanner(strings.NewReader(readLine()))
	sc.Split(bufio.ScanWords)
	N := nextInt(sc)

	sc = bufio.NewScanner(strings.NewReader(readLine()))
	sc.Split(bufio.ScanWords)
	as := make([]int, N)
	for i := range as {
		as[i] = nextInt(sc)
	}

	sc = bufio.NewScanner(strings.NewReader(readLine()))
	sc.Split(bufio.ScanWords)
	bs := make([]int, N)
	for i := range bs {
		bs[i] = nextInt(sc)
	}

	ans := 0
	tmp := make([]int, len(bs))
	for i := uint8(0); i < 31; i++ {
		div := 1 << (i + 1)
		for j, b := range bs {
			tmp[j] = b % div
		}
		sort.Ints(tmp)

		count := 0
		for _, a := range as {
			n := a % div

			x := sort.Search(len(bs), func(j int) bool {
				m := tmp[j]
				return (1<<i)-n <= m
			})
			y := sort.Search(len(bs), func(j int) bool {
				m := tmp[j]
				return 2*(1<<i)-n <= m
			})

			z := sort.Search(len(bs), func(j int) bool {
				m := tmp[j]
				return 3*(1<<i)-n <= m
			})
			w := sort.Search(len(bs), func(j int) bool {
				m := tmp[j]
				return 4*(1<<i)-n <= m
			})

			count += y - x + w - z
		}

		if count%2 == 1 {
			ans += 1 << i
		}
	}

	fmt.Println(ans)
}
