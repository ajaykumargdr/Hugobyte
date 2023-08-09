#!/bin/sh

# if statement
if [ true ]
then
    echo "if statement"
fi

# if else  
a=5
b=10

# Note: if you give 'true' it won't work properly

if [ $a -eq $b ]
then
    echo "if else statement if block"
else 
    echo "if else statement else block"
fi

# if elif else statement

if [ $a -eq $b ]
then
    echo "if - in ladder"
elif [ $a -ne $b ]
then
    echo "elif - in ladder"
else
    echo "else - in ladder"
fi 

# case esac staement

LETTER="i"

case $LETTER in 
    "b") echo "Letter is b"
    ;;

    "c") echo "Letter is c"
    ;;

    "a") echo "Letter is a"
    ;;
    *) echo "Not a valid letter"
       exit 1
esac

