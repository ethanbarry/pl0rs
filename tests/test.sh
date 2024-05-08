#!/bin/sh

echo ╔══════════════════════════╗
echo ║ PL/0 compiler test suite ║
echo ╚══════════════════════════╝
echo ░░░░░░░░░░░░░░░░░░░░░░░░░░░░
echo

# Clear the stderror file...
if [ -e stderror.txt  ] ; then
  rm stderror.txt
fi

for i in *.pl0 ; do
  /usr/bin/printf "%.4s... " $i
  ../target/release/pl-zero-rs -f $i > /dev/null 2>>stderror.txt
  if [ $? -eq 0 ] ; then
    tput setaf 2
    echo ok
    tput sgr0
  else
    echo fail
  fi
done

if [ -s stderror.txt ] ; then
   echo "stderror.txt is not empty!"
   cat stderror.txt
else
   rm stderror.txt
fi
