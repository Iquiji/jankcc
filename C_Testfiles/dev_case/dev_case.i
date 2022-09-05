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

int add(int a,int b){
    puts("add called! :)");
    add(4,7);
    return 7;
}

int main() {
    puts("I was compiled by JankCC!");
    add(4,7);
    return 42;
}
