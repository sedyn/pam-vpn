build-test: test.c
	gcc test.c -o test -lpam -lpam_misc
