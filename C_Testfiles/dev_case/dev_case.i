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
# 18 "C_Testfiles/dev_case/dev_case.c"
int factorial(int x){
    if (x == 0){
        int res = 1;
        printf("%d! = %d",x,res);
    } else {
        int res_2 = x * factorial(x - 1);
        printf("%d! = %d",x,res_2);
    }

    return 1337;
}


int main() {
    puts("I was compiled by JankCC!");
    printf("factorial %d: %d\n",0,factorial(0));
    printf("factorial %d: %d\n",5,factorial(5));
    printf("factorial %d: %d\n",27,factorial(27));
    return 77;
}
