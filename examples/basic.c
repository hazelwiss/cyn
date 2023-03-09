int v0 = 20;
int v1[20];
int v2[30][40];
char *const *v3[20 + 30 * 2];

f0(first, second) int first;
char **const *second;
{ return *second[first]; }

f1() {
    int test1 = v0 + 3;
    test1 += (v0 = test1 - 24) * 3000 << 2;
    return (5 * (2 + 5)) + 3 / test1;
}

int f2(char test) { return (+test / 20) * 256; }

int inline static f3(int argc, char **argv) {
    return +*argv[argc - 1] + 0 [argv];
}

float test_cast(char *to_cast) { return (float)*(int *)to_cast; }

void memset(void *ptr, char val, int len) {
    char *c_beg = ptr;
    char *c_end = c_beg + len;
    while (c_beg < c_end) {
        *c_beg++ = val;
    }
}

int main() {
    v0 >>= 2;
    int result = (f0() - f1()) * (f2(698) / 1000);
    int array[500];
    memset(array, 0, 500 * 4);
    return result * v0 + f3(20, &"hello");
}