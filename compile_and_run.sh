#! /bin/sh
cargo r -- -g -f $1
BASEFILE="$1"
# echo $BASEFILE
BASEFILE_CLEAN="${BASEFILE%.*}"
# echo $BASEFILE_CLEAN
cc "$BASEFILE_CLEAN.o" -o "$BASEFILE_CLEAN.out" -no-pie -static
echo "running compiled file:"
"$BASEFILE_CLEAN.out"
echo "Finished with exit code: $?"