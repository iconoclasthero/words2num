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
