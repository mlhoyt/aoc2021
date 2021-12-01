# aoc2021

https://adventofcode.com/2021/

## Removing the newline at the end of a file

Reference: https://stackoverflow.com/questions/16365155/removing-a-newline-character-at-the-end-of-a-file

Reviewing the last few characters of `{FILE}`:
```
xxd {FILE} | tail -n 2
```

(or `hexdump {FILE} | tail -n 3`)

Removing the trailing newline of `{FILE}`:
```
perl -0pe 's/\n\Z//' {FILE} > /tmp/z
mv /tmp/z {FILE}
```
