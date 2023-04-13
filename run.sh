#!/bin/env bash

set -xe

make install
fplus ${@:2}
