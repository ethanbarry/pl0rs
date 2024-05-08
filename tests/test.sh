#!/bin/sh

echo ╔══════════════════════════╗
echo ║ PL/0 compiler test suite ║
echo ╚══════════════════════════╝
echo ░░░░░░░░░░░░░░░░░░░░░░░░░░░░
echo

for i in *.pl0 ; do
  /usr/bin/printf "%.4s... " $i
  ../target/release/pl-zero-rs -f $i > /dev/null 2>stderror.txt
  if [ $? -eq 0 ] ; then
    echo ok
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
