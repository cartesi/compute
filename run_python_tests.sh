#!/bin/bash
cd ./test
all_test_files=`/bin/ls ./test_*.py | grep -v test_main.py`
python3 -m pytest $all_test_files
