function nth_fib (n) {
    if (n == 1) {
	return 0;
    } else if (n == 2) {
	return 1;
    }
    return nth_fib (n-1) + nth_fib (n-2);
}

print (nth_fib (20))
