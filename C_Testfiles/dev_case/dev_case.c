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

int fib(int num)
{
    if (num == 0) {

        return 0;
    } else if (num == 1) {

        return 1;
    } else {

        return fib(num - 1) + fib(num - 2);
    }
}

int fib_easy(int num){
    // printf("fib_easy called with num=%d\n",num);
    if (num == 0){
        return 0;
    }
    if (num == 1){
        return 1;
    }
    return fib_easy(num - 1) + fib_easy(num - 2);
}

int main() {
    puts("I was compiled by JankCC!");
    // add(4,9);
    // int res = add(4,7);
    // printf("4+7=%d\n",res);
    // printf("7==7: %d\n",7==7);
    // printf("5==8: %d\n",5==8);
    // if (5==5){
    //     puts("if 5==5!");
    // }
    printf("fib_easy 0: %d\n",fib_easy(0));
    printf("fib_easy 1: %d\n",fib_easy(1));
    printf("fib_easy 2: %d\n",fib_easy(2));
    printf("fib_easy 3: %d\n",fib_easy(3));
    printf("fib_easy 4: %d\n",fib_easy(4));
    printf("fib_easy 5: %d\n",fib_easy(5));
    printf("fib_easy 6: %d\n",fib_easy(6));
    printf("fib_easy 7: %d\n",fib_easy(7));
    printf("fib 0: %d\n",fib(0));
    printf("fib 1: %d\n",fib(1));
    printf("fib 2: %d\n",fib(2));
    printf("fib 3: %d\n",fib(3));
    printf("fib 4: %d\n",fib(4));
    printf("fib 5: %d\n",fib(5));
    printf("fib 6: %d\n",fib(6));
    printf("fib 7: %d\n",fib(7));
    return 77;
}

