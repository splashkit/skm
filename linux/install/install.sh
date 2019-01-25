#!/bin/bash

hash lsb_release 2>/dev/null || { echo >&2 "Unable to detect lsb_release - manual install required.  Aborting."; exit 1; }

ID=`lsb_release -is`
VERSION=`lsb_release -rs`

echo "Details coming soon..."