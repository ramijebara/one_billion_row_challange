#/bin/bash

cargo build


directory="test/samples"

shopt -s dotglob

for file in "$directory"/*.txt; do
  echo "testing: $file"

  valid_output=`echo $file | cut -f1 -d'.'`.out
  valid_sha256sum=`cat $valid_output | sha256sum`
  test_sha256sum=`cargo run -q --bin single_threaded -- $file | sha256sum`

  if [[ "$valid_sha256sum" == "$test_sha256sum" ]]; then
    echo "pass"
  else
    echo "fail"
    echo "TEST    `cargo run -q --bin single_threaded -- $file`"
    echo "CORRECT `cat $valid_output`"
  fi
done
