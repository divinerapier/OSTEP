#include <stdlib.h>

int main() {
    int *array = (int *)malloc(sizeof(int) * 100);
    for (int i = 0; i < 100; i++) {
        array[100] = 0;
    }
    return 0;
}