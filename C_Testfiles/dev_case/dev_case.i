# 0 "C_Testfiles/dev_case/dev_case.c"
# 0 "<built-in>"
# 0 "<command-line>"
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 0 "<command-line>" 2
# 1 "././header_fixes/fix.h" 1



typedef char* __builtin_va_list;
# 0 "<command-line>" 2
# 1 "C_Testfiles/dev_case/dev_case.c"
extern int puts (const char *__s);
extern int printf (const char *restrict __format, ...);
# 19 "C_Testfiles/dev_case/dev_case.c"
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
# 54 "C_Testfiles/dev_case/dev_case.c"
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
