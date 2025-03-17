#!/bin/sh

# Just an convenient helper to run cargo watch

cargo watch --no-gitignore -w src -w templates  -x run

#cargo watch --no-gitignore -w 'src/' -x 'run -- --nox 1 --delete orphans --master http://couchdb.salamander-jewelry.net --repl http://cb1.salamander-jewelry.com:5984 --database sl_usa_style'
