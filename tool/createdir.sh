#! /bin/sh

SCRIPT_DIR=$(cd $(dirname $0); pwd)

if [ $# -ne 2 ];then
    echo "createdir.sh needs arguments."
    echo "createdir.sh ContestName ContestNo"
    exit 1
fi

for problem in a b c d
do
    mkdir -p "${SCRIPT_DIR}/../$1/$2/${problem}/01" "${SCRIPT_DIR}/../$1/$2/${problem}/02"
done
