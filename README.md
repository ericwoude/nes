# NES

This repository is a Work In Project. Currently the processor is fully tested, passing all unit tests and the nestest rom. The following steps are lined out in the roadmap below.

## Roadmap
- [x] CPU
  - [x] Fully unit-tested per instruction [(Tom Harte's processor tests)](https://github.com/TomHarte/ProcessorTests/tree/main/6502/v1)
  - [x] Functional tests passing [(nestest)](http://nickmass.com/images/nestest.nes)
- [ ] Mapper 0
- [ ] GPU
- [ ] APU
- [ ] Extended mappers

## Testing
The unit tests are not included in the repository as they sum up to over a GB of data. Invoke `fetch.sh` to download the unit tests.
```bash
λ cd tests
λ sh fetch.sh
λ cargo test
```
