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
    a = 7;
    printf("a.addr=%#018x,a.val=%d\n",&a,a);
    printf("b.addr=%#018x,b.val=%d\n",&b,b);

    long unsigned int malloc_addr = malloc(15);

    puts("I was compiled by JankCC!");
    // printf("factorial %d: %d\n",0,factorial(0));
    // printf("factorial %d: %d\n",5,factorial(5));
    // printf("factorial %d: %d\n",12,factorial(12));
    printf("malloc_addr: %#018x,%d \n",malloc_addr,0);
    int* a_ptr = &a;
    a_ptr = a_ptr + 8;
    int* b_ptr = &b;
    b_ptr = b_ptr - 8;
    printf("a addr: %#018x,deref again: %d\n",a_ptr,*a_ptr);
    printf("b addr: %#018x,deref again: %d\n",&b,*b_ptr);

    return 77;
}

