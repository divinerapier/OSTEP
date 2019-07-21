all:
	gcc ${ARGS} -o null main.c

run: all
	valgrind --leak-check=yes ./null                       

