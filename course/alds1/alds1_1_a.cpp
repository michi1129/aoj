#include <iostream>
#include <vector>
using namespace std;

void trace(int n, vector<int> &a) {
  for (int i = 0; i < n; i++) {
    if (i > 0) cout << " ";
    cout << a[i];
  }
  cout << endl;
}

void insertionSort(int n, vector<int> &a) {
  for (int i = 0; i < n; i++) {
    int v;
    v = a[i];
    int j = i - 1;
    while (j >= 0 && a[j] > v) {
      a[j + 1] = a[j];
      j--;
    }
    a[j + 1] = v;
    trace(n, a);
  }
}

int main() {
  int n;
  cin >> n;
  vector<int> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }
  insertionSort(n, a);

  return 0;
}
