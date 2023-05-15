class MethodCall {
	String world() {
		return "World";
	}

	String hello() {
		return "Hello";
	}

	String f() {
		return hello() + " " + this.world();
	}
}
