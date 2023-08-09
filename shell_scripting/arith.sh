#!/bin/sh

# Arithmetic expressions
val=`expr 2 + 2`

# or we can use this to calculations
# val=$((2 + 2))   # (no space restrictions)

echo "Total value is: $val" 


# With variables

# Multiplication
a=5
b=10
echo "Total values of $a*$b is `expr $a \* $b`" 
#Note: should use escape sequence when using '*'

# Assignment
a=$b
echo "$b"	# 10

# Equality
# echo "${[ $a==$b ]}"

###############################################
# Note: space after bracket is mandatory
# Relational Operators

a=10
b=20

# Equality
# Always showing same
if [ $a==$a ]
then 
   echo "Same =="
fi

# Relational operators (instead <, >, !=, ==)
#Equality
if [ $a -eq $b ]
then 
   echo "Same -eq"
fi

# Not equal
if [ $a -ne $b ]
then 
   echo "Not Same -eq"
fi

# Greater than
if [ $a -lt $b ]
then 
   echo "lt $a $b"
fi

# -lt, -ge, le #options are there

# Boolean Operators

# Not
if [ !false ]
then
    echo "Not value of false"
fi

# Or
if [ true -o false ]
then
    echo "Or value true -o false"
fi

# -a (and alos there)

# String Operators (same as ==)
if [ $a = $b ]
then
    echo "String = operator"
fi

# Unary string operators

# size is zero(1) or not(0) 
if [ -z $b ]
then
    echo "Size is zero"
fi

# -n (non zero), 
if [ -n $b ]
then
    echo "Size is non zero"
fi

# not empty
if [ $b ]
then
    echo "str is not empty"
fi



