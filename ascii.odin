package main;

import "core:fmt"


main :: proc(){

    for i in 0..<257{
      fmt.printf("%c\n",rune(i+33));
    }
}
