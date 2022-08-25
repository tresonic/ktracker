#!/bin/bash
rm -rf ./db
mkdir ./db/
echo ".save ./db/db.db" | sqlite3