#define INT_MIN  (int)((unsigned int)1 << (sizeof(int)*8-1))

double myPow(double x, int n) {
  if (x == 0.0) {
    return 0.0;
  }
  double result = 1.0;
  if (n < 0) {
    if (n == INT_MIN ) {
      n++;
      result *= x;
    }
    n = -n;
    x = 1 / x;
  }
  while (n) {
    if (n & 1) result *= x;
    x *= x;
    n >>= 1;
  }
  return result;
}
