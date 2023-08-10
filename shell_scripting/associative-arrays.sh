#!/bash/sh

TEXT=`cat words.txt` 

declare -A MAP

words=($TEXT)

for word in ${words[@]}
do
    if [ -v ${MAP[$word]} ]
    then
        MAP[$word]=1
        # echo $word
    else
        ((MAP[$word]++)) 
    fi

done

echo ${!MAP[@]}
echo ${MAP[*]}

