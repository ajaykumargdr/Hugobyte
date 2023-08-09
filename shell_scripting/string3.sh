DATARECORD="last=clifford,first=Jhonny Boy,state=CA"

echo ${#DATARECORD}


echo ${DATARECORD[@]/f/l}
echo ${DATARECORD[@]//f/l}


STRING="WWW.ajaykumar@hugobyte.com"

STRING="${STRING[@]/#WWW.}"
echo "${STRING[@]/%.com}"
