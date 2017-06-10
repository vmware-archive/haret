In order to regenerate the protobuf messages for the API the protobuf compiler needs to be
installed.

Follow the [installation
instructions](https://github.com/stepancheg/rust-protobuf#how-to-generate-rust-code) to do this for
your platform.

Then regenerate the `messages.rs` file.

```sh
cd haret/src/api
protoc --rust_out . messages.proto
```
