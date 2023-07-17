git clone -n --depth=1 --filter=tree:0 \
  https://github.com/TomHarte/ProcessorTests/
cd ProcessorTests
git sparse-checkout set --no-cone nes6502/v1
git checkout
mv nes6502/v1/* ../.
rm -rf ProcessorTests

# The tests contain a lot of illegal opcodes which the emulator
# will not support. Therefore, we delete them.
illegal_opcodes="02 03 04 07 0b 0c 0f 12 13 14 17 1a 1b 1c 1f 22 23 27 2b 2f 32 33 34 37 3a 3b 3c 3f 42 43 44 47 4b 4f 52 53 54 57 5a 5b 5c 5f 62 63 64 67 6b 6f 72 73 74 77 7a 7b 7c 7f 80 82 83 87 89 8b 8f 92 93 97 9b 9c 9e 9f a3 a7 ab af b2 b3 b7 bb bf c2 c3 c7 cb cf d2 d3 d4 d7 db dc df e2 e3 e7 eb ef f2 f3 f4 f7 fb fc ff"
for opcode in $illegal_opcodes; do
  rm $opcode.json
done