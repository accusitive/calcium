#include <stdio.h>
int main(int argc, char** argv) {    
    if (argc > 1) {
        printf("> 1\n");
        if ( argc > 5 ) {
            printf("MORE THAN 5 !");
            return 5000;
        }
    }
    return 0;
}