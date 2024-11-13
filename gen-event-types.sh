#!/bin/bash

findtype() {
  files=$(git grep -l "type $1 .* {")
  files=$(git grep -l "type $1 [^{]*$")
  echo "// $1 found in: $(echo $files $files)"

  [ -z "$files" ] || awk "/type $1/" $(echo $files)
  [ -z "$files" ] || awk "/type $1 struct {/,/}/" $(echo $files)
  echo
}

go_to_rust_struct() {
  sed 's/type \(.*\) struct/#[derive(Debug, Clone, Deserialize, Serialize)]\npub struct \1/;
       /^.*`json:"-"`$/d;
       /^[[:space:]]*$/d;
       s/^	\([A-Za-z][^ ]*\) \+\([^\/]*\)/	\1: \2,/;
       s/^	\([A-Za-z][^ ]*\)$/	#[serde(flatten)]\n	\1: \1,/;
       /^	\([a-z][^ ]*\).*:/d;
       /^	\([A-Za-z][^ ]*\).*: ,$/d;
       s/}/}\n/;
       s/\<string\>/String/g;
       s/\<uint64\>/u64/g;
       s/\<uint16\>/u16/g;
       s/\<uint32\>/u32/g;
       s/\<float64\>/f64/g;
       s/\<byte\>/u8/g;
       s/\*//;
       s/\<\([a-z]\+\)\.\([^ ]\+\)\>/\2/;
       s/\<\([a-z]\+\)\.\([^ ]\+\)\>/\1::\2/;
       s/map\[\([A-Za-z]\+\)\]\[\]\([A-Za-z]\+\)/Map<\1,\2>/;
       s/\[\]\[\]\([A-Za-z0-9]\+\)/Vec<Vec<\1>>/;
       s/\[\]\([A-Za-z0-9]\+\)/Vec<\1>/;
       s/^	\([A-Za-z]\+\): \([^ ]\+\).*`json:"\([^,]\+\)"`/	#[serde(rename="\3")]\n	\1: \2/;
       s/^	\([A-Za-z]\+\): \([^ ]\+\).*`json:"\([^,]\+\),omitempty"`/	#[serde(rename="\3")]\n	\1: Option<\2>/;
       # s/^	\([A-Z][^ ]*\):/	pub \1:/;
       ' | sed 's/^	\([A-Z][^ ]*\):/	pub \1:/;' # is there a limit to a number of sed commands?
}

main() {
rm -f /tmp/zitadel-events1.rs
rm -f /tmp/zitadel-events1.go

echo 'package events' > /tmp/zitadel-events1.go
# false &&
for f in $(grep -r 'type .*Event struct' -l | grep internal/repository); do
  modname=$(echo $f | sed 's/internal\/repository\/\(.*\)\.go/\1/; s/[\/]/_/g')
  echo "// { $modname $f"
  echo '/*'
  grep '\(EventType\|Prefix\)' $f | grep '='
  grep -A3 -B1 'NewBaseEventForPush' $f
  echo '*/'
  awk '/.*Event struct {/,/}/' $f
  echo "// } $modname"
  echo
done >> /tmp/zitadel-events1.go

(
echo '#![allow(non_snake_case, missing_docs)]'
echo
echo 'use super::event_types_manual::*;'
echo 'use super::event_types_imports::*;';
echo

cat event-types.list | grep ' ' | while read line; do
  pat='^\([^ ]\+\) *\([^ :]\+\)::\([^ ]\+\)'
  t=$(echo $line | sed "s/$pat/\1/")
  p=$(echo $line | sed "s/$pat/\2/")
  tn=$(echo $line | sed "s/$pat/\3/")
  name=$(echo $t | sed -E 's/[\._]([a-z])/\U\1/g; s/^([a-z])/\U\1/')
  f="internal/repository/$p.go"
  echo "/// \`$t\` event, $f $tn"
  awk "/$tn struct {/,/}/" $f | go_to_rust_struct | sed "s/struct $tn/struct $name/"
done

echo
echo '#[derive(Debug, Clone, Deserialize, Serialize)]'
echo 'pub enum ZitadelEvent {'
cat event-types.list | grep ' ' | while read line; do
  pat='^\([^ ]\+\) *\([^ :]\+\)::\([^ ]\+\)'
  t=$(echo $line | sed "s/$pat/\1/")
  p=$(echo $line | sed "s/$pat/\2/")
  name=$(echo $t | sed -E 's/[\._]([a-z])/\U\1/g; s/^([a-z])/\U\1/')
  echo "  $name($name),"
done
echo '}'
) > /tmp/zitadel-events1.rs

gen_imports > /tmp/zitadel-events2.rs
}

smth() {
  imports_pat='.*: \([a-z]\+\)\.\([A-Za-z]\+\),'
  cat /tmp/zitadel-events1.rs | grep "${imports_pat}" | sed "s/${imports_pat}/\1 \2/"
}

gen_imports() {
  echo '#![allow(non_snake_case, missing_docs)]'
  echo 'use super::event_types_manual::*;'

  echo 'mod nomod {'
  oldpackage=
  cat additional-types.list | while read -r package lepath type; do
    [ "$package" = "$oldpackage" ] || echo -e "}\n\npub mod $package { use super::*;"
    oldpackage=$package

    name=$(echo $package $type | sed -E 's/[\._]([a-z])/\U\1/g; s/^([a-z])/\U\1/')
    awk "/type $type struct {/,/}/" $(grep -l -r "type $type struct" $lepath) | go_to_rust_struct
  done
  echo '}'
}

main
