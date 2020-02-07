if [ ! -f "./db_patch_count" ]; then
  echo "-1" > ./db_patch_count
fi
COUNT=$(cat ./db_patch_count)
COUNT=$(expr ${COUNT} + 0)

if [ -f "./merge.sql" ]; then
  rm ./merge.sql
fi
for filename in ./patches/*.zip; do
  if [ ! -f "${filename}" ]; then
    continue
  fi

  FILE=${filename:10}
  VERSION=$(expr ${FILE:0:5} + 0)

  if (( ${VERSION} > ${COUNT} )); then
    unzip ${filename}
    cat ${filename:10:-4}.sql >> merge.sql
    echo "" >> merge.sql
    echo ${VERSION} > ./db_patch_count
    rm ${filename:10:-4}.sql
  fi
done