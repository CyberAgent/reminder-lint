#!/bin/sh -l

/usr/local/bin/reminder-lint/cli "$@" > /tmp/reminder-lint.log 2>&1

echo "stdout<<EOF" >> $GITHUB_OUTPUT
cat /tmp/reminder-lint.log >> $GITHUB_OUTPUT
echo "EOF" >> $GITHUB_OUTPUT
