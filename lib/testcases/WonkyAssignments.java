class WonkyAssignments {
	int x, y = 3, z;
	boolean a = false, b = true, c;

	int f(int newX) {
		c = (x = newX) > y;
		int i, j = z = x, k = -1;
		a = !(b = c == true);
		if (a) return z;
		else return k;
	}
}