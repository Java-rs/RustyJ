class MethodCall {
	String world() {
		return "World";
	}
	String hello() {
		return "Hello";
	}
	String f() {
		return (this.hello()) + ((" ") + (this.world()));
	}
}
