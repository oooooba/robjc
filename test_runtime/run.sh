#!/bin/bash

set -eu

RUST_BACKTRACE=1

RUNTIME=../target/debug/librobjc.so

cargo build

function build_and_test(){
    src=$1
    exe=/tmp/$1.exe
    if [ ! -f $exe ]; then
        cc -o $exe $src $RUNTIME
    fi
    if $exe; then
        echo "passed: $src"
    else
        echo "failed: $src"
        exit
    fi
}

build_and_test _cmd.m
build_and_test accessing_ivars.m
build_and_test bycopy-1.m
build_and_test class_self-1.m
build_and_test class_self-2.m
build_and_test class-1.m
build_and_test class-2.m
build_and_test class-3.m
build_and_test class-4.m
build_and_test class-5.m
build_and_test class-6.m
build_and_test class-7.m
build_and_test class-8.m
build_and_test class-9.m
build_and_test class-10.m
build_and_test class-11.m
build_and_test class-12.m
build_and_test class-13.m
build_and_test class-14.m
build_and_test IMP.m
build_and_test object_is_class.m
build_and_test object_is_meta_class.m
build_and_test redefining_self.m
build_and_test root_methods.m
build_and_test selector-1.m
build_and_test static-1.m
build_and_test static-2.m
build_and_test trivial.m
build_and_test va_method.m
