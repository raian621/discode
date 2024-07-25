#!/usr/bin/env fish

# Exports key value pairs in an env file into the environment variables of the
# current shell session.

if test -z "$argv[1]"
  echo "Usage: $argv[0] <env file path>"
  exit
end

for line in (cat $argv[1] | grep -v '^#')
  set item (string split -m 1 '=' $line)
  set -gx $item[1] (echo $item[2] | sed -E 's/"(.*)"/\1/g')
end
