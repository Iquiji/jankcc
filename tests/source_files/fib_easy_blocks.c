#import <stdio.h>

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
    printf("fib_easy 27: %d\n",fib_easy(27));
    return 0;
}

