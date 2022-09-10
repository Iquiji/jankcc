typedef long unsigned int size_t;
extern int puts (const char *__s);
extern int printf (const char *restrict __format, ...);
extern void *malloc(size_t size);

int fib(int num){
    int result;
    if (num == 0) {

        result = 0;
    } else if (num == 1) {

        result = 1;
    } else {

        result = fib(num - 1) + fib(num - 2);
    }
    // printf("fib(%d) => %d \n",num,result);
    return result;
}

int factorial(int x){
    int res;
    if (x == 0){
        res = 1;
        
    } else {
        res = x * factorial(x - 1);
    }
    // printf("%d! => %d\n",x,res);
    return res;
}


int main() {
    int a = 5;
    int b = 0;
    b = 1;
    // printf("a=%d,b=%d\n",a,b);
    a = 7;

    long unsigned int malloc_addr = malloc(15);

    puts("I was compiled by JankCC!");
    // printf("factorial %d: %d\n",0,factorial(0));
    // printf("factorial %d: %d\n",5,factorial(5));
    // printf("factorial %d: %d\n",12,factorial(12));
    printf("malloc_addr: %#018x \n",malloc_addr);
    printf("a addr: %#018x \n",&a);
    printf("b addr: %#018x \n",&b);

    return 77;
}

