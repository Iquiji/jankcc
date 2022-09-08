extern int puts (const char *__s);
extern int printf (const char *restrict __format, ...);

// int main() {
//     return 42;
// }


int add(int a,int b){
    int c = a + b;
    //printf("add called! :) a: %d b: %d c: %d \n",a,b,c);
    return c;
}

int dumb_func(int wow){
    return wow;
}

int main() {
    // puts("I was compiled by JankCC!");
    // add(4,9);
    // int res = add(4,7);
    // printf("4+7=%d\n",res);
    // printf("7==7: %d\n",7==7);
    // printf("5==8: %d\n",5==8);
    if (5==5){
        puts("if 5==5!");
    }
    return 77;
}

