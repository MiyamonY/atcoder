#! /bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)

if [ $# -ne 3 ];then
    echo "createdir.sh needs arguments."
    echo "createdir.sh ContestName ContestNo No"
    exit 1
fi

for problem in a b c d e f
do
    mkdir -p "${SCRIPT_DIR}/../$1/$2/${problem}/$3"
    (cd "${SCRIPT_DIR}/../$1/$2/${problem}/$3"; dune init executable "$3" --libs str && dune build)
done
