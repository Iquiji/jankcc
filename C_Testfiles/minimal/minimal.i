# 0 "C_Testfiles/minimal/minimal.c"
# 0 "<built-in>"
# 0 "<command-line>"
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 0 "<command-line>" 2
# 1 "././header_fixes/fix.h" 1



typedef char* __builtin_va_list;
# 0 "<command-line>" 2
# 1 "C_Testfiles/minimal/minimal.c"
extern int puts (const char *__s);

int main() {
    puts("Hello World!");
    puts("I was compiled by JankCC!");
    puts("");
    puts("");
    puts("minimal example works! :)");
    return 42;
}
