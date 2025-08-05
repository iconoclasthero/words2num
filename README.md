# words2num

Convert written English number words to integers. Supports:

- Full parsing: `four sixty seven` → `467`
- Additive parsing: `four hundred sixty seven` → `467`
- Replacement mode: `"Chapter Twenty-One"` → `"Chapter 21"`
- Optional zero-padding: `--pad -3` → `004`
- Stdin or CLI input

## Examples

```bash
words2num "twenty twenty-five"           # 2025
words2num -4 "nineteen eighty-four"      # 1984
echo "Chapter Twenty-One" | words2num --replace
# → Chapter 21
```

## Compiling
```
clone repo
cd words2num/src
cargo build --release
cp ../target/release/words2num <destination>
```

## Converting ABS `metadata.json` 

- To convert the AudioBookShelf chapter index in the metadata.json to 2-digit numerals:

```
readarray -t titles < <(jq -r '.chapters[].title' updated.json | words2num --replace -2)
jq --argjson newtitles "$(printf '%s\n' "${titles[@]}" | jq -R . | jq -s .)" '
  .chapters |= [range(0; length) as $i | .[$i].title = $newtitles[$i] | .[$i]]
' metadata.json > updated.json
jq -r '.chapters[].title' updated.json
```
- After **verifying** changes are successfull, (and perhaps after a `cp metadata.json metadata.json-$(date)` to make a backup) `mv updated.json metadata.json` 
