#!/bin/bash

if type sfz >/dev/null 2>&1; then
  sfz ./
else
  echo 'no sfz, installing'
  cargo install sfz
fi

