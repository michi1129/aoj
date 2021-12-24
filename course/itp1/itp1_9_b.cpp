#include <iostream>
using namespace std;
int main() {
  string s;
  int m;
  int h;
  while (1) {
    cin >> s;
    if (s == "-") {
      break;
    }

    cin >> m;
    int t = 0;
    for (int i = 0; i < m; ++i) {
      cin >> h;
      t += h;
    }
    t %= s.length();

    if (t == 0) {
      cout << s << endl;
    } else {
      for (int i = t; i < s.length(); ++i) {
        cout << s[i];
      }
      for (int i = 0; i < t; ++i) {
        cout << s[i];
      }
      cout << endl;
    }
  }
  return 0;
}
