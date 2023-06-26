class While {
	int n = 2;

	int f(int x) {
		int i = 0;
		int a = n;
		while (i < x) {
			a = a + a;
			i = i + 1;
		}
		return a;
	}
}
