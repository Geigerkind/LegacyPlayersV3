if [ -d ./Database ]; then
  cp ./Database/db_patch_count ./
  rm -r ./Database
fi