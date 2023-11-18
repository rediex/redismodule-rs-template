#!/bin/bash

ROOT_DIR=$(cd $(dirname $0);pwd)
TEST_DIR=$ROOT_DIR/tests/pytest
LIB_DIR=$ROOT_DIR/target/debug

cd $TEST_DIR

if test -f $LIB_DIR"/lib{{crate_name}}.dylib"; then
  bash tests.sh $LIB_DIR/lib{{crate_name}}.dylib
elif test -f $LIB_DIR"/lib{{crate_name}}.dylib"; then
  bash tests.sh $LIB_DIR/lib{{crate_name}}.so
else
  echo "Not exist so/dylib file"
fi


