# rsunimrcp-sys
Rust binding of [UniMRCP framework](https://github.com/unispeech/unimrcp).

## Build
Build script depends on the UniMRCP library and its dependencies all of which must be available at `UNIMRCP_PATH` and `APR_LIB_PATH`. Also build script needs forked by UniMRCP APR include file at `APR_INCLUDE_PATH`. Please refer to [UniMRCP installation instructions](https://github.com/unispeech/unimrcp/blob/master/INSTALL).
```bash
$ cargo build --release
```
