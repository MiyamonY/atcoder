///
// File:  d.cpp
// Author: ymiyamoto
//
// Created on Sat Dec 29 17:23:13 2018
//
#include <bits/stdc++.h>
using namespace std;

#define rep(_n, _init, _N) for (int32_t _n = _init; _n < (int32_t)(_N); _n++)
#define rrep(_n, _N, _end) for (int32_t _n = (int32_t)(_N); _n >= _end; _n--)
#define iceil(_x, _y) (((_x) + (_y)-1) / (_y))
#define ifloor(_x, _y) ((_x) / (_y))

static const double pi = 2 * asin(1);
static const double precision = 10e-9;
#define int long long int

int32_t main()
{
  int n;
  cin >> n;
  vector<int> a(n);
  vector<int> b(n);
  rep(i, 0, n){
    cin >> a[i];
  }
  rep(i, 0, n){
    cin >> b[i];
  }

  vector<int> modb(n);
  int ans = 0;
  rep(i, 0, 32){
    rep(j, 0, n){
      modb[j] = b[j] % (1 << (i+1));
    }
    sort(modb.begin(), modb.end());

    int num = 0;
    rep(j, 0, n){
      int moda = a[j] % (1 << (i+1));
      auto x = lower_bound(modb.begin(), modb.end(), 1*(1<<i) - moda);
      auto y = lower_bound(modb.begin(), modb.end(), 2*(1<<i) - moda);
      auto z = lower_bound(modb.begin(), modb.end(), 3*(1<<i) - moda);
      auto w = lower_bound(modb.begin(), modb.end(), 4*(1<<i) - moda);
      num += y-x + w-z;
      // cout << j << " " << num << " " << distance(modb.begin(), x) << " "<<distance(modb.begin(), y)
      //      << " "<<distance(modb.begin(), z) << " "<<distance(modb.begin(), w) << endl;
    }

    if(num % 2 == 1){
      ans |= (1 << i);
    }
  }
  cout << ans << endl;

  return 0;
}
