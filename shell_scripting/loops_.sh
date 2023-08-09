#!/bin/sh

# while

x=20

# while [ $x -gt 10 ]
# do
#    echo $x
#    x=`expr $x - 1`
# done

# until

# until [ $x -gt 30 ]
# do
#    echo $x
#    x=`expr $x + 1`
# done

# for

for i in {0..5}
do
    echo $i

    for j in {0..5}
    do
        echo $j
	if [ $j -eq 3 ]
	then
	    break 2    # 2nd outer loop
        fi
    done
done

