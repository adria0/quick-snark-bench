template T(N) {
	signal private input p;
	signal output q;
	signal tmp[N];

	tmp[0] <== 1;
	for(var i=1;i<N;i++) {
		tmp[i] <== tmp[i-1]*p;
	}
	q <== tmp[N-1];
}
component main = T(50000);
