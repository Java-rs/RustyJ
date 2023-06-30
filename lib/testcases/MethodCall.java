class MethodCall {
	int a() {
		return 2;
	}

	int b() {
		return 5;
	}

	int f() {
		return a() + this.b();
	}
}
