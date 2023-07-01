class ArithmeticMethods {
	int x = 69;
	int y = 420; // Too big for signed byte (important for codegen)
	int bigInt = 131072; // Too big for signed short, which (important for codegen)

	int addX(int a) {
		return x + a;
	}

	int addY(int a) {
		return y + a;
	}

	int complexMath(int a, int b) {
		a = y * (a + b) / x;
		b = a + (-b);
		a = x + b * a;
		return x * a + bigInt;
	}
}
