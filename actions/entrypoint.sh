#!/bin/sh -l
if [ "$1" = "run" ]; then
  echo "::add-matcher::.github/matcher.json"
fi

/usr/local/bin/reminder-lint/cli "$@"

if [ "$1" = "run" ]; then
  echo "::remove-matcher owner=reminder-lint::"
fi
