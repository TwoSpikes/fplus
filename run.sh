#!/bin/env bash

set -e

make install
fplus ${@:1}
