#!/bin/env bash

set -e;

start_from=1;
debug='false';
if [[ $1 == --in-main-rs ]] then
	let "start_from+=2";
fi;
if [[ $1 == --debug ]] || [[ $3 == --debug ]] then
	let "start_from+=1";
	debug='true';
fi;
if [[ $debug == 'false' ]] then
	make fplus;
elif [[ $debug == 'true' ]] then
	make debug;
fi;
make install;
fplus ${@:$start_from};

set errorcode $?;
unset run;
exit $errorcode;
