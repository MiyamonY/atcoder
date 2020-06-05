#! /bin/bash

mkdir $1
if [ "$1" = "aoj" ]; then 
   (cd $1; dune init executable $1 && dune build)
else
   (cd $1; dune init  executable $1 --libs batteries && dune build)
fi

