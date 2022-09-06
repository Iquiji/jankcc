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

int add(int a,int b){
    int c = a + b;

    return c;
}

int main() {
    puts("I was compiled by JankCC!");
    add(4,9);
    int res = add(4,7);
    printf("4+7=%d\n",res);
    return add(70,7);
}
