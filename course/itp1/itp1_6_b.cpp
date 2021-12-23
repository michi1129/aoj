#include <iostream>
#include <map>
using namespace std;
int main() {
  int n;
  char suite;
  int rank;
  map<int, int> s;
  map<int, int> h;
  map<int, int> c;
  map<int, int> d;
  for (int i = 0; i < n; ++i) {
    cin >> suite >> rank;
    switch (suite) {
      case 'S':
        s[rank] = 1;
        break;
      case 'H':
        h[rank] = 1;
        break;
      case 'C':
        c[rank] = 1;
        break;
      case 'D':
        d[rank] = 1;
        break;
    }
  }
  for (int i = 1; i <= 13; ++i) {
    if (s[i] != 1) {
      cout << "S " << i << endl;
    }
  }
  for (int i = 1; i <= 13; ++i) {
    if (h[i] != 1) {
      cout << "H " << i << endl;
    }
  }
  for (int i = 1; i <= 13; ++i) {
    if (c[i] != 1) {
      cout << "C " << i << endl;
    }
  }
  for (int i = 1; i <= 13; ++i) {
    if (d[i] != 1) {
      cout << "D " << i << endl;
    }
  }
  return 0;
}
