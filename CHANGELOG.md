# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## 2.0.0 (2024-12-24)


### ⚠ BREAKING CHANGES

* This PR alters the API
* the parameter is changed on public methods.
* this will affect current chunked data
* correct max chunk size to 500kiB
* reducing the max chunk size from 1MB to 500kB
* Public API changes as SecretKey is renamed to DataMap.
* **docs:** Complete change of API.
* **deps:** underlying deps were updated
* **self_encryptor.rs:** As this makes the previous data unusable, this is a breaking change.
* **test/lib.rs:** Last commit changed the argument types of the SE API
* **self_encryptor.rs:** Remove truncate api from self encryptor
* **tokio:** new Tokio runtime version is not backward compatible with tokio versions < 1.
* **self_encryptor.rs:** Previous commit (c6aafe6) introduced a breaking change due to introduction of new delete trait.

### Features

* Enhance Python bindings and CLI interface ([ec85e38](https://github.com/dirvine/self_encryption/commit/ec85e38d9565084d896ab8ddb3a22a857f48fa7b))
* **cli:** Streamline CLI and improve documentation ([87d7b70](https://github.com/dirvine/self_encryption/commit/87d7b70e2184c37ab2a0a063fec152d8c6151402))
* add command line arguments to parallel decryptor example ([53d5c8d](https://github.com/dirvine/self_encryption/commit/53d5c8d1581e07f8eac98dcea3e4bd5013b31c7d))
* **python:** Add streaming encryption and update documentation ([73b508b](https://github.com/dirvine/self_encryption/commit/73b508bbeb1223d5e57ac276f3e04292fc2361ae))
* inital commit of functor based encrypt ([f7941ed](https://github.com/dirvine/self_encryption/commit/f7941ed291a88c08a145cc27f176744dbcb0c6b3))
* **docs:** enhance Python bindings documentation and CLI ([996361e](https://github.com/dirvine/self_encryption/commit/996361ee48a6421a8589c53743e5d281c768d618))
* patch bump ([79119ad](https://github.com/dirvine/self_encryption/commit/79119ad5dd605adf5d7138a359d81f715167ceb0))
* **cli:** Add command-line interface for self-encryption ([0633f5b](https://github.com/dirvine/self_encryption/commit/0633f5bb6eb521fa3a8210374ba67a4dce50941a))
* **data_map:** Add hierarchical data map shrinking and expansion ([b80f1cd](https://github.com/dirvine/self_encryption/commit/b80f1cd5c3c0a0468377a0330c6b4bb1b8f76c1b))
* **docs:** Add comprehensive Python documentation and docstrings ([59c05c8](https://github.com/dirvine/self_encryption/commit/59c05c804ea7f1ee6e89df90d511389debe583a6))
* **verify:** Add Chunk Verification and Clean Up Python Bindings ([322371d](https://github.com/dirvine/self_encryption/commit/322371d325aef962475ae34f2b1b77691ceddecd))
* add python bindings using PyO3 ([b39f875](https://github.com/dirvine/self_encryption/commit/b39f875f9469c047eaf0a92e346bac5373b6abd7))
* allow compile time override of MAX_CHUNK_SIZE ([1a4334f](https://github.com/dirvine/self_encryption/commit/1a4334f2d1c313d56363be578dcdad92ff8466c4))
* delete for Self-Encryptor ([7480376](https://github.com/dirvine/self_encryption/commit/74803764955e3fc46771012ab81f2fee3ea59668))
* encrypt from file ([b7f0a38](https://github.com/dirvine/self_encryption/commit/b7f0a384eff22382da8418b239adcb2bf2d0f086))
* encrypt/decrypt to/from disk files ([d585909](https://github.com/dirvine/self_encryption/commit/d5859091c648fdcfbf0b09dcb44da577c76bfab6))
* PYPI Token ([50fb896](https://github.com/dirvine/self_encryption/commit/50fb89651bc4f056d3a93bd902b0438d93dbbab0))
* PYPI Token ([e834b99](https://github.com/dirvine/self_encryption/commit/e834b998ef273db5d754e1521c0bb40f9a31a7c1))
* PYPI Token / linux/mac/win ([d4bb999](https://github.com/dirvine/self_encryption/commit/d4bb999a47f807e3d8d8efca6add1d510bf356c7))
* reduce MIN_CHUNK_SIZE and MIN_ENCRYPTABLE_BYTES down to 1/3 bytes. ([11f6a70](https://github.com/dirvine/self_encryption/commit/11f6a70e6c2a68085612b2c96d6aadca3c8c4f20))
* reduce tokio dep requirements ([5f1ab1a](https://github.com/dirvine/self_encryption/commit/5f1ab1a3ccd6af6d190cdd3e1580ed0a874fe9a3))
* test pythno bindings ([ce5c188](https://github.com/dirvine/self_encryption/commit/ce5c188175f79d54fd7fb499ceb5bd8276439ab1))
* test python bindings ([47ca12f](https://github.com/dirvine/self_encryption/commit/47ca12f054d1eb1d36d267738b1a38676a9f8324))
* update breaking change ([73aff89](https://github.com/dirvine/self_encryption/commit/73aff89037396efdb08fe403943fafa751735428))
* **audit:** add scheduled security audit scan ([28508f1](https://github.com/dirvine/self_encryption/commit/28508f190ab3bc48ad4ecbb9941f23ceae378e43))
* **python:** Enhance Python Bindings, Add Streaming Decrypt, and Improve Testing ([bff658f](https://github.com/dirvine/self_encryption/commit/bff658f36da80ef26e622f42e622c378bf908ddc))
* **storage:** add recursive data map retrieval for decryption ([91cb3a4](https://github.com/dirvine/self_encryption/commit/91cb3a4102e53349035ff63b1d5d67052ca4081b))
* add optional child field to DataMap ([69071c1](https://github.com/dirvine/self_encryption/commit/69071c1db0a52f30c9ba9d39ce8337a0fb152b0a))
* PYPI Token / linux/mac/win ([a1f0959](https://github.com/dirvine/self_encryption/commit/a1f0959416383a719ad98a5e7e3e6786ce3a82a4))
* PYPI Token / linux/mac/win test 5 ([eba957e](https://github.com/dirvine/self_encryption/commit/eba957e0d5f92fe13675d01526fe05221b4a0b0e))
* PYPI Token / linux/mac/win test 6 ([229658c](https://github.com/dirvine/self_encryption/commit/229658c924711269e41bd82c23a7eb4bd79bf18a))
* PYPI Token / linux/mac/win test 7 ([32bcf4e](https://github.com/dirvine/self_encryption/commit/32bcf4e9683ec6499ea288511126ee454e527ffb))
* PYPI Token / linux/mac/win test 8 ([ef2a18f](https://github.com/dirvine/self_encryption/commit/ef2a18fa0ffdd85356abd8603247922e86abcd18))
* reducing the max chunk size from 1MB to 500kB ([7577322](https://github.com/dirvine/self_encryption/commit/7577322d9288ce516a9e56dec3aa0977be45e1ac))
* stream encryptor write encrypted chunks to disk ([5f3f906](https://github.com/dirvine/self_encryption/commit/5f3f906804cd2e6871f0f2e982977244afa5e3ec))
* stream self encryptor ([abdc7c1](https://github.com/dirvine/self_encryption/commit/abdc7c11a404b65109009759ed13f30443f37737))
* test full python bindings ([753c0f0](https://github.com/dirvine/self_encryption/commit/753c0f01e6411a88bc1de2e9b1bea3c0d74ba09e))
* test python bindings ([7ef8f31](https://github.com/dirvine/self_encryption/commit/7ef8f316d86bb15f4d39cd5d4e17b1ae82d8fc8c))
* version bump and fmt ([b65d196](https://github.com/dirvine/self_encryption/commit/b65d196fd856c8156bfae0aecc92326cde27289e))
* **ci:** add cargo publish to CI ([80c614f](https://github.com/dirvine/self_encryption/commit/80c614ff72bfe733e815c40d125f536fdd770667))
* **errors:** add specific for too few bytes ([4b572e9](https://github.com/dirvine/self_encryption/commit/4b572e9857529d76707d6e304d62d9a048850d42))
* **hierarchical:** Add hierarchical data maps and flexible storage backends ([92895a5](https://github.com/dirvine/self_encryption/commit/92895a54299bdab8a9097b6f49576eb97dadfc3c))
* Python full bindings ([521854f](https://github.com/dirvine/self_encryption/commit/521854f4144908a2c443a0464ae3ffc4ed2c7c0f))
* **get:** self mut for get api ([1dfeca3](https://github.com/dirvine/self_encryption/commit/1dfeca3715604612aea982fa9c795413e4b443f5))
* **read:** perform reading from storage in parallel for faster reads ([e760063](https://github.com/dirvine/self_encryption/commit/e76006387a24c88bb3acaacdc2484dcd1c6068f2))
* **reading:** implement faster reading ([26799d9](https://github.com/dirvine/self_encryption/commit/26799d93658cbd3b11478aea1ad482ecdb8f25bf))
* **self_encryptor.rs:** Remove truncate api from self encryptor ([77b2f57](https://github.com/dirvine/self_encryption/commit/77b2f57331624396cf21c484f477c5101d4ea207))
* **self_encryptor.rs:** Store chunks on write ([bed44c4](https://github.com/dirvine/self_encryption/commit/bed44c446bfb364e48c9a17a70d6fb5399723685))
* **storage:** delete trait for SEStorage ([056c4b7](https://github.com/dirvine/self_encryption/commit/056c4b7d4cd63dc3d32a7de46338099de817915a))


### Bug Fixes

* bind ([37b65e4](https://github.com/dirvine/self_encryption/commit/37b65e4a52ee44f2cdffc7a20b21c1cf69a56e27))
* bindings ([51f220f](https://github.com/dirvine/self_encryption/commit/51f220f0692bc4d2af0dd347540ab5c6c59a7348))
* bindings ([5ca2589](https://github.com/dirvine/self_encryption/commit/5ca25898b1408c257cb8d83b80087fba5b46551b))
* bindings ([2bbf9a2](https://github.com/dirvine/self_encryption/commit/2bbf9a254ecc0c9181198f2ea474d6525f34d0c0))
* bindings ([35a7e77](https://github.com/dirvine/self_encryption/commit/35a7e77a87487172dd31b39deb6707e0b04c3745))
* bindings ([af184d0](https://github.com/dirvine/self_encryption/commit/af184d019d0c0644b0f04e46cd3483fa27420198))
* bindings ([1f08f3e](https://github.com/dirvine/self_encryption/commit/1f08f3e6be13de720a000122c929505a6e2a88a5))
* bindings ([fc941b2](https://github.com/dirvine/self_encryption/commit/fc941b2939303c040304a85c73f4e551949068b2))
* bindings ([7c8f971](https://github.com/dirvine/self_encryption/commit/7c8f971f2f6e081d8acff728b97391c37b6e15fa))
* bindings ([c9a50eb](https://github.com/dirvine/self_encryption/commit/c9a50ebee88d2af8387fa79aa9ffac97ccaa6ac5))
* bindings ([8f97674](https://github.com/dirvine/self_encryption/commit/8f9767401df73ef2ffc6d40382abfbf5183f02c1))
* bindings ([11c92e8](https://github.com/dirvine/self_encryption/commit/11c92e8d9a198fcc541c08b33b5a3b7f945b56c2))
* bindings ([545a4b0](https://github.com/dirvine/self_encryption/commit/545a4b015c4c316926c406d2b7ccb3d42c73e6ca))
* bindings ([45be3f6](https://github.com/dirvine/self_encryption/commit/45be3f672f7b503d60c2d790fcb464d1c533575f))
* clippy ([50887f3](https://github.com/dirvine/self_encryption/commit/50887f3d26c853bd067fe6696d3ed7e9245ab4aa))
* clippy and fmt ([49f3bd9](https://github.com/dirvine/self_encryption/commit/49f3bd97160d5540186bbcd3b837835f549508e1))
* correct max chunk size to 500kiB ([15f41a0](https://github.com/dirvine/self_encryption/commit/15f41a06c1845c9a5afc29d04c3b153b6dfdc889))
* dep check ([a9d4bae](https://github.com/dirvine/self_encryption/commit/a9d4bae7a18de6acd8762e251f28fb81f4f01fa9))
* enforce min size, sort keys in new ([e7f4ec0](https://github.com/dirvine/self_encryption/commit/e7f4ec0eef1343c234e7b69143048250523504d2))
* ensure decrypter targeting file shall not be pre-existing ([dbeacce](https://github.com/dirvine/self_encryption/commit/dbeaccedf9a1be4d6f0ebfb4229eb810c48a6b07))
* fmt ([0fa2363](https://github.com/dirvine/self_encryption/commit/0fa2363e810f7598d946aab728779d1847788b1f))
* improve chunk handling and add comprehensive tests ([853ffe6](https://github.com/dirvine/self_encryption/commit/853ffe6c76766ef90be83aa855c47b1bd5207298))
* improve Python bindings and add comprehensive tests ([64df179](https://github.com/dirvine/self_encryption/commit/64df17919fbaae033ea49838ec121b8fb589f80a))
* mark Boxed Future as Send ([232166d](https://github.com/dirvine/self_encryption/commit/232166d7ab422dbd4bd88ff442d5a722aba4904a))
* minor clippy fix ([479adba](https://github.com/dirvine/self_encryption/commit/479adbaf350d6a473c0c2558c54fe4afdf4d2989))
* prevent panics in decrypt_full_set and decrypt_range ([58107d8](https://github.com/dirvine/self_encryption/commit/58107d8f332e03d2f371c33b3bcc5069e79a9f63))
* remove unwanted file ([b92bace](https://github.com/dirvine/self_encryption/commit/b92bace5c3642f66df916c6ccb89dbb970bdfb66))
* update benchmarks to use public API ([1d4f79a](https://github.com/dirvine/self_encryption/commit/1d4f79ad36644efa1f06fc114a6e672cfae59ed1))
* windows bindings ([8eeadbb](https://github.com/dirvine/self_encryption/commit/8eeadbb697caea0061ea44204a8c17ca8e6ce80c))
* workflow ([52ed3de](https://github.com/dirvine/self_encryption/commit/52ed3de542c2241e720fa8133a0100ab72b44381))
* **benches:** update for min size change ([940f6fe](https://github.com/dirvine/self_encryption/commit/940f6fefb461fa98bd76b125eedb921bc80f749f))
* **Cargo.toml:** Remove rustc-serialize dependency ([a9baf6b](https://github.com/dirvine/self_encryption/commit/a9baf6bb9bab44ce67530610dc6f9a5eacd27831))
* **clippy:** fix clippy errors ([7dc38d1](https://github.com/dirvine/self_encryption/commit/7dc38d180c1b715730d6100cf67d13072c5ef1c2))
* **decrypt:** prevent extra clones while decrypting chunks ([9f77326](https://github.com/dirvine/self_encryption/commit/9f77326bdf477d467ae992478bdc4a795286ef97))
* **encryption:** fix CBC padding issues in chunk decryption ([84613e8](https://github.com/dirvine/self_encryption/commit/84613e8a84d9a317578e5e02c98f34de1a1c5c37))
* **range:** allow to pass ranges with length overflowing data length ([919ed53](https://github.com/dirvine/self_encryption/commit/919ed53a4146da30b58ad9483cd98df85d3a4343))
* **seek:** avoid range out of bounds panic ([fcd56b9](https://github.com/dirvine/self_encryption/commit/fcd56b9e33df2efa1632ef74030301b1a30bdc2f))
* **seek:** use correct offset ([34f327d](https://github.com/dirvine/self_encryption/commit/34f327df86fce937613ecf33b864517689cf3573))
* **self_encryptor.rs:** Fix edge case of writing ([f212e88](https://github.com/dirvine/self_encryption/commit/f212e8828cdfd3c519a811bf4c6a9af5e3ccd037))
* **self_encryptor.rs:** Fix edge cases in truncate logic ([de3ea50](https://github.com/dirvine/self_encryption/commit/de3ea50d5f8131c557c0bfe4ec3cd92c11989b5a))
* **self_encryptor.rs:** Requested Changes from the review ([993b524](https://github.com/dirvine/self_encryption/commit/993b524e0d8c01d537ef9960e5e711b612653e7d))
* **self_encryptor.rs:** Set IV to second half of last chunk hash ([61852dc](https://github.com/dirvine/self_encryption/commit/61852dcb73240fd6ce91dd412c269740166f607c))
* **tests/lib.rs:** Make different data size for 32 and 64 bit binaries ([c6e5ca1](https://github.com/dirvine/self_encryption/commit/c6e5ca121f577b824539cb7cc8e6e84f1ea5e7ed))
* **writes:** process network writes in parallel ([4974a95](https://github.com/dirvine/self_encryption/commit/4974a95d7117e796aafcc973124c3033b4c5fb04))


### update

* **deps:** update deps flagged by security audit ([df8f2ac](https://github.com/dirvine/self_encryption/commit/df8f2ac3cd048706d5237f10f1d08d97d4f6ea59))


* remove usage of Box<PathBuf> ([2a88d98](https://github.com/dirvine/self_encryption/commit/2a88d98acd51e0cebcc2c2d09287041354a0cf68))
* rename secretkey to datamap ([75f4131](https://github.com/dirvine/self_encryption/commit/75f41311c2b3c562826f3646792774d4fbc9a728))
* **docs:** add explainer for IV and Pad. ([2d56d1a](https://github.com/dirvine/self_encryption/commit/2d56d1a9a7999b562e59934a1e825a4a05e0dfa4))
* **self_encryptor.rs:** Use Err inplace of panic! and expect ([a4cae07](https://github.com/dirvine/self_encryption/commit/a4cae07a1ff530c987513e5bba937c31e5c64d55))
* **test/lib.rs:** Add more crossplatform tests ([136be7f](https://github.com/dirvine/self_encryption/commit/136be7fd58b7a23c4522244938492c44c7b27b25))
* **tokio:** upgrading to v1.3.0 ([640593b](https://github.com/dirvine/self_encryption/commit/640593b1fbbe3d8f67c2ae730584ddbf6060703c))

### [0.29.2](https://github.com/maidsafe/self_encryption/compare/v0.29.1...v0.29.2) (2024-04-22)

### [0.29.1](https://github.com/maidsafe/self_encryption/compare/v0.29.0...v0.29.1) (2024-01-24)


### Features

* reduce tokio dep requirements ([5f1ab1a](https://github.com/maidsafe/self_encryption/commit/5f1ab1a3ccd6af6d190cdd3e1580ed0a874fe9a3))

## [0.30.0](https://github.com/maidsafe/self_encryption/compare/v0.29.2...v0.30.0) - 2024-09-26

### Added
- allow compile time override of MAX_CHUNK_SIZE

### Other
- update error derivation crates
- add test doc to please udeps
- *(pr)* change code coverage generation
- *(merge)* remove auto-merge (dependabot)
- cargo fmt
- apply cargo clippy --fix
- remove box_pointers lint
- cargo fmt
- [**breaking**] remove usage of Box<PathBuf>

## [0.29.0](https://github.com/maidsafe/self_encryption/compare/v0.28.6...v0.29.0) (2024-01-10)


### ⚠ BREAKING CHANGES

* this will affect current chunked data

### Features

* reduce MIN_CHUNK_SIZE and MIN_ENCRYPTABLE_BYTES down to 1/3 bytes. ([11f6a70](https://github.com/maidsafe/self_encryption/commit/11f6a70e6c2a68085612b2c96d6aadca3c8c4f20))

### [0.28.6](https://github.com/maidsafe/self_encryption/compare/v0.28.5...v0.28.6) (2023-12-19)


### Bug Fixes

* **decrypt:** prevent extra clones while decrypting chunks ([9f77326](https://github.com/maidsafe/self_encryption/commit/9f77326bdf477d467ae992478bdc4a795286ef97))

### [0.28.5](https://github.com/maidsafe/self_encryption/compare/v0.28.4...v0.28.5) (2023-10-09)


### Bug Fixes

* ensure decrypter targeting file shall not be pre-existing ([dbeacce](https://github.com/maidsafe/self_encryption/commit/dbeaccedf9a1be4d6f0ebfb4229eb810c48a6b07))

### [0.28.4](https://github.com/maidsafe/self_encryption/compare/v0.28.3...v0.28.4) (2023-09-07)


### Features

* stream encryptor write encrypted chunks to disk ([5f3f906](https://github.com/maidsafe/self_encryption/commit/5f3f906804cd2e6871f0f2e982977244afa5e3ec))

### [0.28.3](https://github.com/maidsafe/self_encryption/compare/v0.28.2...v0.28.3) (2023-09-06)


### Features

* stream self encryptor ([abdc7c1](https://github.com/maidsafe/self_encryption/commit/abdc7c11a404b65109009759ed13f30443f37737))

### [0.28.2](https://github.com/maidsafe/self_encryption/compare/v0.28.1...v0.28.2) (2023-09-05)


### Features

* encrypt/decrypt to/from disk files ([d585909](https://github.com/maidsafe/self_encryption/commit/d5859091c648fdcfbf0b09dcb44da577c76bfab6))

### [0.28.1](https://github.com/maidsafe/self_encryption/compare/v0.28.0...v0.28.1) (2023-09-04)


### Features

* encrypt from file ([b7f0a38](https://github.com/maidsafe/self_encryption/commit/b7f0a384eff22382da8418b239adcb2bf2d0f086))

## [0.28.0](https://github.com/maidsafe/self_encryption/compare/v0.27.5...v0.28.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* correct max chunk size to 500kiB
* reducing the max chunk size from 1MB to 500kB

### Features

* reducing the max chunk size from 1MB to 500kB ([7577322](https://github.com/maidsafe/self_encryption/commit/7577322d9288ce516a9e56dec3aa0977be45e1ac))


### Bug Fixes

* correct max chunk size to 500kiB ([15f41a0](https://github.com/maidsafe/self_encryption/commit/15f41a06c1845c9a5afc29d04c3b153b6dfdc889))

## [0.28.0](https://github.com/maidsafe/self_encryption/compare/v0.27.5...v0.28.0) (2023-03-15)


### ⚠ BREAKING CHANGES

* correct max chunk size to 500kiB
* reducing the max chunk size from 1MB to 500kB

### Features

* reducing the max chunk size from 1MB to 500kB ([7577322](https://github.com/maidsafe/self_encryption/commit/7577322d9288ce516a9e56dec3aa0977be45e1ac))


### Bug Fixes

* correct max chunk size to 500kiB ([15f41a0](https://github.com/maidsafe/self_encryption/commit/15f41a06c1845c9a5afc29d04c3b153b6dfdc889))

## [0.28.0](https://github.com/maidsafe/self_encryption/compare/v0.27.5...v0.28.0) (2023-02-22)


### ⚠ BREAKING CHANGES

* reducing the max chunk size from 1MB to 500kB

### Features

* reducing the max chunk size from 1MB to 500kB ([7577322](https://github.com/maidsafe/self_encryption/commit/7577322d9288ce516a9e56dec3aa0977be45e1ac))

### [0.27.5](https://github.com/maidsafe/self_encryption/compare/v0.27.4...v0.27.5) (2022-08-09)

### [0.27.4](https://github.com/maidsafe/self_encryption/compare/v0.27.3...v0.27.4) (2022-03-18)

### [0.27.3](https://github.com/maidsafe/self_encryption/compare/v0.27.2...v0.27.3) (2022-02-25)

### [0.27.2](https://github.com/maidsafe/self_encryption/compare/v0.27.1...v0.27.2) (2022-02-24)


### Bug Fixes

* minor clippy fix ([479adba](https://github.com/maidsafe/self_encryption/commit/479adbaf350d6a473c0c2558c54fe4afdf4d2989))

### [0.27.1](https://github.com/maidsafe/self_encryption/compare/v0.27.0...v0.27.1) (2021-12-14)


### Bug Fixes

* **range:** allow to pass ranges with length overflowing data length ([919ed53](https://github.com/maidsafe/self_encryption/commit/919ed53a4146da30b58ad9483cd98df85d3a4343))

## [0.27.0](https://github.com/maidsafe/self_encryption/compare/v0.26.3...v0.27.0) (2021-09-22)


### ⚠ BREAKING CHANGES

* Public API changes as SecretKey is renamed to DataMap.

* rename secretkey to datamap ([75f4131](https://github.com/maidsafe/self_encryption/commit/75f41311c2b3c562826f3646792774d4fbc9a728))

### [0.26.3](https://github.com/maidsafe/self_encryption/compare/v0.26.2...v0.26.3) (2021-09-13)


### Features

* **errors:** add specific for too few bytes ([4b572e9](https://github.com/maidsafe/self_encryption/commit/4b572e9857529d76707d6e304d62d9a048850d42))


### Bug Fixes

* **seek:** avoid range out of bounds panic ([fcd56b9](https://github.com/maidsafe/self_encryption/commit/fcd56b9e33df2efa1632ef74030301b1a30bdc2f))

### [0.26.2](https://github.com/maidsafe/self_encryption/compare/v0.26.1...v0.26.2) (2021-09-03)

### [0.26.1](https://github.com/maidsafe/self_encryption/compare/v0.26.0...v0.26.1) (2021-08-31)


### Bug Fixes

* **seek:** use correct offset ([34f327d](https://github.com/maidsafe/self_encryption/commit/34f327df86fce937613ecf33b864517689cf3573))

## [0.26.0](https://github.com/maidsafe/self_encryption/compare/v0.25.0...v0.26.0) (2021-08-30)


### ⚠ BREAKING CHANGES

* **docs:** Complete change of API.

### Features

* **reading:** implement faster reading ([26799d9](https://github.com/maidsafe/self_encryption/commit/26799d93658cbd3b11478aea1ad482ecdb8f25bf))


### Bug Fixes

* **benches:** update for min size change ([940f6fe](https://github.com/maidsafe/self_encryption/commit/940f6fefb461fa98bd76b125eedb921bc80f749f))
* enforce min size, sort keys in new ([e7f4ec0](https://github.com/maidsafe/self_encryption/commit/e7f4ec0eef1343c234e7b69143048250523504d2))


* **docs:** add explainer for IV and Pad. ([2d56d1a](https://github.com/maidsafe/self_encryption/commit/2d56d1a9a7999b562e59934a1e825a4a05e0dfa4))

## [0.25.0](https://github.com/maidsafe/self_encryption/compare/v0.24.3...v0.25.0) (2021-07-06)


### ⚠ BREAKING CHANGES

* **deps:** underlying deps were updated

### Bug Fixes

* **writes:** process network writes in parallel ([4974a95](https://github.com/maidsafe/self_encryption/commit/4974a95d7117e796aafcc973124c3033b4c5fb04))


### update

* **deps:** update deps flagged by security audit ([df8f2ac](https://github.com/maidsafe/self_encryption/commit/df8f2ac3cd048706d5237f10f1d08d97d4f6ea59))

### [0.24.3](https://github.com/maidsafe/self_encryption/compare/v0.24.2...v0.24.3) (2021-06-24)


### Bug Fixes

* mark Boxed Future as Send ([232166d](https://github.com/maidsafe/self_encryption/commit/232166d7ab422dbd4bd88ff442d5a722aba4904a))

### [0.24.2](https://github.com/maidsafe/self_encryption/compare/v0.24.1...v0.24.2) (2021-06-08)

### [0.24.1](https://github.com/maidsafe/self_encryption/compare/v0.24.0...v0.24.1) (2021-05-04)


### Features

* **read:** perform reading from storage in parallel for faster reads ([e760063](https://github.com/maidsafe/self_encryption/commit/e76006387a24c88bb3acaacdc2484dcd1c6068f2))

## [0.24.0](https://github.com/maidsafe/self_encryption/compare/v0.23.1...v0.24.0) (2021-04-28)


### ⚠ BREAKING CHANGES

* **self_encryptor.rs:** As this makes the previous data unusable, this is a breaking change.

### Bug Fixes

* **self_encryptor.rs:** Set IV to second half of last chunk hash ([61852dc](https://github.com/maidsafe/self_encryption/commit/61852dcb73240fd6ce91dd412c269740166f607c))

### [0.23.1](https://github.com/maidsafe/self_encryption/compare/v0.23.0...v0.23.1) (2021-04-26)

## [0.23.0](https://github.com/maidsafe/self_encryption/compare/v0.22.0...v0.23.0) (2021-04-13)


### ⚠ BREAKING CHANGES

* **test/lib.rs:** Last commit changed the argument types of the SE API

### Bug Fixes

* **tests/lib.rs:** Make different data size for 32 and 64 bit binaries ([c6e5ca1](https://github.com/maidsafe/self_encryption/commit/c6e5ca121f577b824539cb7cc8e6e84f1ea5e7ed))


* **test/lib.rs:** Add more crossplatform tests ([136be7f](https://github.com/maidsafe/self_encryption/commit/136be7fd58b7a23c4522244938492c44c7b27b25))

## [0.22.0](https://github.com/maidsafe/self_encryption/compare/v0.21.0...v0.22.0) (2021-04-05)


### ⚠ BREAKING CHANGES

* **self_encryptor.rs:** Remove truncate api from self encryptor

### Features

* **self_encryptor.rs:** Remove truncate api from self encryptor ([77b2f57](https://github.com/maidsafe/self_encryption/commit/77b2f57331624396cf21c484f477c5101d4ea207))
* **self_encryptor.rs:** Store chunks on write ([bed44c4](https://github.com/maidsafe/self_encryption/commit/bed44c446bfb364e48c9a17a70d6fb5399723685))


### Bug Fixes

* **Cargo.toml:** Remove rustc-serialize dependency ([a9baf6b](https://github.com/maidsafe/self_encryption/commit/a9baf6bb9bab44ce67530610dc6f9a5eacd27831))
* **self_encryptor.rs:** Fix edge case of writing ([f212e88](https://github.com/maidsafe/self_encryption/commit/f212e8828cdfd3c519a811bf4c6a9af5e3ccd037))
* **self_encryptor.rs:** Fix edge cases in truncate logic ([de3ea50](https://github.com/maidsafe/self_encryption/commit/de3ea50d5f8131c557c0bfe4ec3cd92c11989b5a))
* **self_encryptor.rs:** Requested Changes from the review ([993b524](https://github.com/maidsafe/self_encryption/commit/993b524e0d8c01d537ef9960e5e711b612653e7d))

## [0.21.0](https://github.com/maidsafe/self_encryption/compare/v0.20.2...v0.21.0) (2021-03-11)


### ⚠ BREAKING CHANGES

* **tokio:** new Tokio runtime version is not backward compatible with tokio versions < 1.

* **tokio:** upgrading to v1.3.0 ([640593b](https://github.com/maidsafe/self_encryption/commit/640593b1fbbe3d8f67c2ae730584ddbf6060703c))

### [0.20.2](https://github.com/maidsafe/self_encryption/compare/v0.20.1...v0.20.2) (2021-03-03)

### [0.20.1](https://github.com/maidsafe/self_encryption/compare/v0.20.0...v0.20.1) (2021-02-25)

## [0.20.0](https://github.com/maidsafe/self_encryption/compare/v0.19.11...v0.20.0) (2021-02-22)


### ⚠ BREAKING CHANGES

* **self_encryptor.rs:** Previous commit (c6aafe6) introduced a breaking change due to introduction of new delete trait.

### Features

* delete for Self-Encryptor ([7480376](https://github.com/maidsafe/self_encryption/commit/74803764955e3fc46771012ab81f2fee3ea59668))
* **storage:** delete trait for SEStorage ([056c4b7](https://github.com/maidsafe/self_encryption/commit/056c4b7d4cd63dc3d32a7de46338099de817915a))


* **self_encryptor.rs:** Use Err inplace of panic! and expect ([a4cae07](https://github.com/maidsafe/self_encryption/commit/a4cae07a1ff530c987513e5bba937c31e5c64d55))

### [0.19.11](https://github.com/maidsafe/self_encryption/compare/v0.19.10...v0.19.11) (2021-02-15)

### [0.19.10](https://github.com/maidsafe/self_encryption/compare/v0.19.9...v0.19.10) (2021-02-10)

### [0.19.9](https://github.com/maidsafe/self_encryption/compare/v0.19.8...v0.19.9) (2021-02-10)

### [0.19.8](https://github.com/maidsafe/self_encryption/compare/v0.19.7...v0.19.8) (2021-02-03)

### [0.19.7](https://github.com/maidsafe/self_encryption/compare/v0.19.6...v0.19.7) (2021-01-20)

### [0.19.6](https://github.com/maidsafe/self_encryption/compare/v0.19.5...v0.19.6) (2021-01-18)

### [0.19.5](https://github.com/maidsafe/self_encryption/compare/v0.19.4...v0.19.5) (2020-11-23)

### [0.19.4](https://github.com/maidsafe/self_encryption/compare/v0.19.3...v0.19.4) (2020-11-23)

### [0.19.3](https://github.com/maidsafe/self_encryption/compare/v0.19.2...v0.19.3) (2020-10-20)

### [0.19.2](https://github.com/maidsafe/self_encryption/compare/v0.19.1...v0.19.2) (2020-10-09)

### [0.19.1](https://github.com/maidsafe/self_encryption/compare/v0.19.0...v0.19.1) (2020-09-21)


### Features

* **get:** self mut for get api ([1dfeca3](https://github.com/maidsafe/self_encryption/commit/1dfeca3715604612aea982fa9c795413e4b443f5))

### [0.19.0](https://github.com/maidsafe/self_encryption/compare/v0.18.0...v0.19.0) (2020-07-30)

* Update rand and rand_chacha dep

### [0.18.0](https://github.com/maidsafe/self_encryption/compare/v0.17.0...v0.18.0) (2020-06-26)

* Update bincode dep
* Update deps > v1 in general to use implicit ^

### [0.17.0](https://github.com/maidsafe/self_encryption/compare/v0.16.0...v0.17.0) (2020-05-28)

* Update to use modern rust futures
* Use async/await throughout
* Use Arc/Mutex to enable multi-threading


### [0.16.0](https://github.com/maidsafe/self_encryption/compare/v0.15.0...v0.16.0) (2019-12-02)

* Replace the use of `rust_sodium` with `aes` for encryption.

### [0.15.0](https://github.com/maidsafe/self_encryption/compare/0.14.0...v0.15.0) (2019-08-29)

* Update rand to 0.6.0
* Remove the legacy maidsafe_utilities dependency
* Update memmap to 0.7.0 and remove the unsafe code
* Add `generate_address` function to the `Storage` trait to support data types with different address deriving algorithms
* Use rust stable / edition 2018

### [0.14.0]

* Update tiny_keccak to 1.4.0

### [0.13.0]
* Upgrade unwrap version to 1.2.0
* Use rust 1.28.0 stable / 2018-07-07 nightly
* rustfmt 0.99.2 and clippy-0.0.212
* Update license to mention GPL3 only
* Replace the brotli2 library with a pure Rust version

### [0.12.0]
* Use rust 1.22.1 stable / 2017-11-23 nightly
* rustfmt 0.9.0 and clippy-0.0.174

### [0.11.2]
* Update rust_sodium to 0.6.0

### [0.11.1]
* Update futures to latest version and fix deprecated function calls

### [0.11.0]
* Use rust 1.19 stable / 2017-07-20 nightly
* rustfmt 0.9.0 and clippy-0.0.144
* Replace -Zno-trans with cargo check
* Make appveyor script using fixed version of stable

### [0.10.0]
* Self-encrypt is now asyc using futures

### [0.9.0]
* Use sha3_256 from tiny_keccak instead of rust_sodium
* Travis uses cargo_install script from QA
* Dependencies updated

### [0.8.0]
* Update maidsafe_utilities 0.11.0
* rustfmt 0.8.1
* switch to serde instead of rustc-serialize
* cleanup CI scripts

### [0.7.1]
* Update maidsafe_utilities to v0.10.0 which removes deprecated API's.

### [0.7.0]
* Use new rust_sodium crate instead of sodiumoxide.

### [0.6.0]
* Expose a new SequentialEncryptor which publishes its data immediately if possible.

### [0.5.1]
* Fix sodiumoxide to v0.0.10 as the new released v0.0.12 does not support rustc-serializable types anymore and breaks builds

### [0.5.0]
* Use SHA256 instead of SHA512.

### [0.4.0]
* Remove asynchronous code.
* Replace Deflate compression with Brotli.
* Use `Result`s instead of panic.

### [0.3.1]
* Fix truncate, flagging first two chunks for encryption, and add new test.
* Updates contributor agreement.
* Fixed failing test exceeding serialisation limits.
* Disable clippy use_debug check.
* Updated dependencies.

### [0.3.0]
* Updated dependencies.

### [0.2.6]
* Various bug fixes and tidy up.
* Setup clippy usage.
* Include nightly builds on travis.

### [0.2.5]
* Swap forked memory_map for original memmap crate.

### [0.2.4]
* Remove wildcards from dependencies.

### [0.2.3]
* Update in line with sodiumoxide 0.0.9 changes.

### [0.2.2]
* Increase file sizes to 1Gb using memory map (previously omitted).
* Compression pre encrypt and post encrypt in encrypt and decrypt methods
* Task passing to allow cores to be lit up when handling chunks

### [0.2.1]
* Fixed lint warnings caused by latest Rust nightly

### [0.0.0 - 0.2.0]
* Initial structure
* Test set-up
* Travis integration
* Docs creation
* Docs hosting (github.io)
* Windows CI set-up (ci.AppVeyor.com)
* Read/Write file in memory based buffer
* API version 0.0.8
* Implement disk based interface as example
* Full unit tests in lib.rs
* Integrations tests in tests module
* Benchmark tests for varying file sizes from 1 byte to 10 M/b
* API stable version 0.1.0
* Coverage analysis (coveralls ?)
