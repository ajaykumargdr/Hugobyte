#!/bin/sh

<<kaments 
# this multi line comment name 'kaments' can be anything
# multi line comments
#"<<comment name"
#...code block
#same_name

# kaments

##############################
# echo "what is your name?"
# read PERSON
# echo "Hello, $PERSON"

##############################

echo "The present working directory is"
pwd
echo "List of file in this directory is"
ls

# kaments

#######################Variables#########################
# always have UPPERCASE as UNIX convention

#_ALI = 1  # WILL NOT WORK , should not use space
TOKEN_A=200
VAR_1="Zara Ali"
VAR_2='K'
BOOL_VAL=true 
VAL_2=thisVal 

echo $TOKEN_A $VAR_1 $VAR_2 $BOOL_VAL $VAL_2

#######
# kaments
##################Read Only Variables#######################

NAME="Will make it constant"
readonly NAME
# NAME="Cannot be changed" 

echo "$NAME"

##########
kaments

####################Unset Variables######################
NAME="Zara Ali"
# unset NAME	# value is unset now
echo $NAME 	# will not print anything

###################
##############################
##############################
##############################
##############################
##############################
