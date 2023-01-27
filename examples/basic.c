int hello = 0;

int main(int argc, char **argv) {
    int test = hello + 1;
    test = test -= ~test %= 20 * 3;
    return test * 3 >> 2;
}