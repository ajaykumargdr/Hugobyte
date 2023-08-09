#!/bin/sh

echo -e "ssdf \c"     # c - no new line
echo -e "backspace\b"	# not working
echo -e "\a"		# for alert
echo -e "\h \v"		# not working
echo -e "tab\tspace"	# working

# substituion commands

up=`date ; uptime`
d=`pwd`
echo "working in $d on $up"

# variable substitution

x=10

echo "${x}"	# 10

unset x
echo ${x:-"message is replaced because x is unset"}
echo -e "Og value of x is $x \n\n"

echo ${x:="this message is set to x because x is unset"}
echo "Og value of x is $x"

unset x
echo ${x:+"defaul word"}	# not working
echo "'x:+' $x"

