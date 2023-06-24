class Fib {
	int rec(int n) {
		if (n < 2) {
			return n;
		} else {
			return rec(n - 1) + rec(n - 2);
		}
	}

	int iter(int n) {
		if (n < 2) {
			return n;
		}
		int x = 0;
		int y = 1;
		int i = 1;
		while (i < n) {
			int next = y + x;
			x = y;
			y = next;
			i = i + 1;
		}
		return y;
	}
}
