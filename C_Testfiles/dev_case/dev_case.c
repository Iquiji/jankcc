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

