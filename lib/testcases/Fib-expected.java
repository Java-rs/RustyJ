class Fib {
	int rec(int n) {
		if ((n) < (2)) {
			return n;
		} else {
			return (this.rec((n) - (1))) + (this.rec((n) - (2)));
		}
	}
	int iter(int n) {
		if ((n) < (2)) {
			return n;
		}
		int x;
		x = 0;
		int y;
		y = 1;
		int i;
		i = 1;
		while ((i) < (n)) {
			int next;
			next = (y) + (x);
			x = y;
			y = next;
			i = (i) + (1);
		}
		return y;
	}
}
