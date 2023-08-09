#!/bin/sh

# Arrays
NAME=("Zara" "Qadir" "Ayan" "Daisy" "Mahnaz")
echo ${NAME[0]} ${NAME[1]} ${NAME[2]} ${NAME[3]} ${NAME[4]}
echo "all *- ${NAME[*]}"
echo "all @- ${NAME[@]}"
echo "all ${#NAME[*]}"
echo "all ${#NAME[@]}"

##############################################
# working

NAME={"Zara","Qadir","Ayan","Daisy","Mahnaz"}

# this is not working (because single value)
echo ${NAME[0]}	# Bad substitution error

# this loop

for VALUE in $NAME
do 
    echo "value: $VALUE"
done

# echo "First Method: ${NAME[*]}"  # Bad substitution error
# echo "Second Method: ${NAME[@]}" # "		"  

# printed like this
# {Zara, Qadir, Ayan, Daisy, Mahnaz}
###############################################
