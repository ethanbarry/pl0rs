#!/bin/sh

echo PL/0 compiler test suite
echo ========================

for i in *.pl0 ; do
  /usr/bin/printf "%.4s... " $i
  ../target/release/pl-zero-rs < $i > /dev/null 2>&1
  if [ $? -eq 0 ] ; then
    echo ok
  else
    echo fail
  fi
done
