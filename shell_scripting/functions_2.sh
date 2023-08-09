#!/bin/sh


hello(){
    echo "Hello world"

    echo "$@"

    return 25
} 


hello arg1 arg2

echo $?
