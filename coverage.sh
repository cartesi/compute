#!/bin/bash
forge coverage --report lcov
genhtml -o report --branch-coverage lcov.info
