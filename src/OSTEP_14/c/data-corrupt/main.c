#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    int *array = (int *)malloc(sizeof(int) * 100);
    memset(array, 10, sizeof(int) * 100);
    int index = 10;
    int value1 = 10 << 24 | 10 << 16 | 10 << 8 | 10;
    printf("index: %d, value1: %d, value2: %d\n", index, value1, array[index]);
    free(array);
    printf("index: %d, value1: %d, value2: %d\n", index, value1, array[index]);
    return 0;
}