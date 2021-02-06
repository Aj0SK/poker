#include <bits/stdc++.h>

#define ff first
#define ss second

using namespace std;

using ll = long long;
using pll = pair<long long, long long>;

vector<vector<int>> generated;

void f(vector<int> generating, int sum)
{
  if (sum == 0)
  {
    while (generating.size() != 13)
      generating.push_back(0);
  }
  if (generating.size() == 13)
  {
    if (sum == 0)
      generated.push_back(generating);

    return;
  }

  for (int i = 0; i <= min(sum, 4); ++i)
  {
    generating.push_back(i);
    f(generating, sum - i);
    generating.pop_back();
  }
}

int main()
{
  ios_base::sync_with_stdio(false);
  vector<int> h;
  f(h, 7);

  cout << "Size is " << generated.size() << "\n";

  for (auto x : generated)
  {
    cout << "Moznost je:";
    for (auto i : x)
      cout << " " << i;
    cout << "\n";
  }

  return 0;
}