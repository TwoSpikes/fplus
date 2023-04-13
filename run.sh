#!/bin/env bash

set -e

make install

run() {
	set from $0;
	set all $1;
	fplus ${all:$from};
	unset from all;
	return $?;
}

if [[ $1 == --in-main-rs ]] then
	run 2 $@;
else
	run 1 $@;
fi

set errorcode $?;
unset run;
exit $errorcode;
