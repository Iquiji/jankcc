extern int printf (const char *restrict __format, ...);

int add(int a,int b){
    int c = a + b;
    return c;
}

int main() {
    int res = add(4000,777);
    printf("4000+777=%d",res);
    return 0;
}

