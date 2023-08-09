STRING_VAL="This is a string"

# print string value
echo ${STRING_VAL}

# length of a string
echo "Length of the string is ${#STRING_VAL}"

# find index of s (non zero index)  
STRING="this is a string"
SUBSTRING="hat"
expr index "$STRING" "s"  # arguments should be string

# Substring extraction (zero index)
STRING="this is a string"
echo ${STRING:0:4}	# 0 from 0+4
echo ${STRING:1}	# 1 to end

############### String data extraction ##################

DATARECORD="last=clifford,first=Jhonny Boy,state=CA"
COMMA1=`expr index "$DATARECORD" ','`
CHOP1FIELD=${DATARECORD:$COMMA1}
COMMA2=`expr index "$CHOP1FIELD" ','`

# Length of first name ("Jhonny Boy")
LENGTH=$(($COMMA2 - 6 -1))    # 6 for "first=" and 1 for ","
FIRSTNAME=${CHOP1FIELD:6:$LENGTH}

# Got first name
echo $FIRSTNAME $LENGTH

############### String data Replacement ##################
# will not change the actual data

# Replace first occurance 
STRING="! to be or not to be"
echo "Original String is: $STRING"
echo "${STRING[@]/be/eat}"     # to eat or not to be

# Replace all occurances of substring
echo ${STRING[@]//be/eat}    # to eat or not to eat

# Delete all occurances
echo ${STRING[@]// be}

# Replace occurance of substring only *if at the beginning
echo ${STRING[@]/to be/eat now}   # changed
echo ${STRING[@]/#to be/eat now}  # not changed cz no occur

# Replace subtring if at the end
STRING="@gmail.com"
echo ${STRING[@]/#@/''}	# gmail.com
echo ${STRING[@]/%.com/''}   # @gmail

 
