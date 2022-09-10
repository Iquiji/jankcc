#import <stdio.h>

int fib(int num)
{
    if (num == 0) {

        return 0;
    } else if (num == 1) {

        return 1;
    } else {

        return fib(num - 1) + fib(num - 2);
    }
    // this is here because blocks have to be filled :/ idk how to solve that right now
    puts("unreachable! :/");
    return 7;
}

int main() {
    printf("fib 27: %d\n",fib(27));
    return 0;
}
