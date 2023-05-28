class ComplexIf {
	boolean f(char c) {
		if ((c) == ('a')) {
			return true;
		} else {
			if ((c) == ('b')) {
				return false;
			} else {
				if ((c) == ('c')) {
					return true;
				} else {
					if (((c) == ('d')) || ((c) == ('e'))) {
						return false;
					} else {
						if (((c) == ('d')) || ((c) == ('e'))) {
							return true;
						} else {
							return false;
						}
					}
				}
			}
		}
	}
}
