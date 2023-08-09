#!/bin/sh

# Basic function

function_1(){
    echo "This is a function"
    echo "This function is now called"
}

function_1

# with parameters

function_with_paras(){
    echo -n "These are the parameters were passed"
    echo " $*"
    echo "Number of parameters passed $#"
    echo "N'th element in argument $n"
}

function_with_paras arg1 arg2 arg3

# with returning

function_with_return(){
    echo "This function will return a value 8"
    return 8
}

function_with_return
echo "Returned value from the function is $?"   # $?

# 
