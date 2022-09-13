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
int bored_malloc(int x){
    malloc(1024000);
    puts("malloc'd 1024B");
    return bored_malloc(x + 1);
}

int main() {
    puts("I was compiled by JankCC!");

    int a = 1;
    int b = 2;
    int c = 3;
    int d = 4;
    int e = 5;
    int f = 6;
    int g = 7;
    long long_var = 5;
    if (long_var == 5){
        puts("long_var == 5");
    }
    b = 2;
    a = 1;
    printf("a.addr=%#018x,a.val=%d\n",&a,a);
    printf("b.addr=%#018x,b.val=%d\n",&b,b);

    int* malloc_addr = malloc(16);

    // printf("factorial %d: %d\n",0,factorial(0));
    // printf("factorial %d: %d\n",5,factorial(5));
    // printf("factorial %d: %d\n",12,factorial(12));
    printf("malloc_addr: %#018x,stored_data: %d \n",malloc_addr,*malloc_addr);
    int* a_ptr = &a;
    a_ptr = a_ptr + 8 * 5;
    int* b_ptr = &b;
    b_ptr = b_ptr - 8;
    printf("a addr: %#018x,deref again: %d\n",a_ptr,*a_ptr);
    printf("b addr: %#018x,deref again: %d\n",b_ptr,*b_ptr);
    printf(a_ptr,a_ptr,*b_ptr);

    // bored_malloc(0);
    int* var_ptr = &a;
    for(int x = 0; x < 10; x = x + 1){
        puts("hello for_loop!");
        printf("addr: %#018x,deref + x: %d\n",var_ptr,*var_ptr);
        var_ptr = var_ptr + 8;
    }

    return 77;
}

