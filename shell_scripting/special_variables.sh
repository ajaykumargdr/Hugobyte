#!/bin/sh

# echo $O

echo "First argument- $1"
echo "Second argument- $2"
echo "third argument- $3"
echo "The number of arguments supplied- $#"
echo "all arguments in as single variable- $*"
echo "individual double quated arg list- $@"
echo "Process number(executing command number) - $$" 

# echo "command status of last command- $?"
# echo "$!"

########## Looping through arguments

for TOKEN in $*
do
    echo $TOKEN
done

#################
for TOEKN in $@
do
    echo " '$@' $TOKEN"	 # Not getting expected 
done


