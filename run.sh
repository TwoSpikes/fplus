#!/bin/env bash

set -e

make install

if [[ $1 == --in-main-rs ]] then
	fplus ${@:2};
else
	fplus ${@:1};
fi

set errorcode $?;
unset run;
exit $errorcode;
