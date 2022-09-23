#! /bin/sh
# get Base name without extension
BASEFILE="$1"
BASEFILE_CLEAN="${BASEFILE%.*}"
# remove old files if existant
rm "$BASEFILE_CLEAN.o"
rm "$BASEFILE_CLEAN.out"
# compile C code to object file
cargo r -- -g $1 || exit 1
# link Object file to a static binary
cc "$BASEFILE_CLEAN.o" -o "$BASEFILE_CLEAN.out" -no-pie -static || exit 1
# run compiled file
echo "running compiled file:"
echo "----------------------"
"$BASEFILE_CLEAN.out"
echo "Finished with exit code: $?"