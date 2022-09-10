extern int puts (const char *__s);
extern int printf (const char *restrict __format, ...);

// int main() {
//     return 42;
// }


// int add(int a,int b){
//     int c = a + b;
//     //printf("add called! :) a: %d b: %d c: %d \n",a,b,c);
//     return c;
// }

// int dumb_func(int wow){
//     return wow;
// }
int factorial(int x){
    int res;
    if (x == 0){
        res = 1;
        
    } else {
        res = x * factorial(x - 1);
    }
    printf("%d! => %d\n",x,res);
    return res;
}


int main() {
    int a = 5;
    int b = 0;
    b = 1;
    printf("a=%d,b=%d\n",a,b);
    a = 7;
    puts("I was compiled by JankCC!");
    printf("factorial %d: %d\n",0,factorial(0));
    printf("factorial %d: %d\n",5,factorial(5));
    printf("factorial %d: %d\n",12,factorial(12));
    return 77;
}

