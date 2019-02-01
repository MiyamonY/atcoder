///
// File:  main.cpp
// Author: ymiyamoto
//
// Created on Fri Feb  1 15:28:02 2019
//
#include <bits/stdc++.h>
using namespace std;

#define rep(_n, _init, _N) for (int32_t _n = _init; _n < (int32_t)(_N); _n++)
#define rrep(_n, _N, _end) for (int32_t _n = (int32_t)(_N); _n >= _end; _n--)
#define iceil(_x, _y) (((_x) + (_y)-1) / (_y))
#define ifloor(_x, _y) ((_x) / (_y))

static const double pi = 2 * asin(1);
static const double precision = 10e-9;

int32_t main()
{
  int64_t n;
  scanf("%ld", &n);

  vector<int64_t> as(n);
  for(int i = 0; i < n;i++){
    scanf("%ld", &as[i]);
  }
  vector<int64_t> bs(n);
  for(int i = 0; i < n;i++){
    scanf("%ld", &bs[i]);
  }

  int64_t ans = 0;
  vector<int64_t> tmp(n);
  for(int i = 0; i < 31;i++){
    int64_t div = 1 << (i+1);
    for(int j = 0; j < n;j++){
      tmp[j] = bs[j] % div;
    }

    sort(tmp.begin(), tmp.end());
    int64_t count  = 0;
    for(int j = 0;j <n;j++){
      int64_t a = as[j] % div;
      auto x = lower_bound(tmp.begin(),tmp.end(), (1<<i)-a);
      auto y = lower_bound(tmp.begin(),tmp.end(), 2*(1<<i)-a);
      auto z = lower_bound(tmp.begin(),tmp.end(), 3*(1<<i)-a);
      auto w = lower_bound(tmp.begin(),tmp.end(), 4*(1<<i)-a);
      count += y - x + w - z;
    }
    if(count %2 == 1){
      ans += 1<<i;
    }
  }
  printf("%ld\n", ans);

  return 0;
}
