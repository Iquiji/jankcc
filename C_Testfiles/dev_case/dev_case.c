extern int puts (const char *__s);
extern int printf (const char *restrict __format, ...);

int add(int a,int b){
    int c = a + b;
    //printf("add called! :) a: %d b: %d c: %d \n",a,b,c);
    return c;
}

int main() {
    puts("I was compiled by JankCC!");
    add(4,9);
    int res = add(4,7);
    printf("4+7=%d\n",res);
    return add(70,7);
}

