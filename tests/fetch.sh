git clone -n --depth=1 --filter=tree:0 \
  https://github.com/TomHarte/ProcessorTests/
cd ProcessorTests
git sparse-checkout set --no-cone nes6502/v1
git checkout
mv nes6502/v1/* ../.
rm -rf ProcessorTests