#!/bin/bash

cd native

cargo build

cd ..

rm -rf native_libs

mkdir native_libs

cp native/target/release/libhypua_node_native* native_libs

yarn build