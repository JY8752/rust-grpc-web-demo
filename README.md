# rust-grpc-web-demo

あとから気付いたがprotovalidateがRust対応していなかった。なので、このリポジトリの実装は無駄ではあるがBufでFileDescriptorSetを生成してtonic(prost)でコード生成する流れは正常に動いたので一応サンプルとして残しておきます。

## buf

```bash
buf --version
1.57.0
```

以下で`buf.yaml`生成。

```bash
buf config init
```

protovalidate使いたいので依存として持ってくる

```bash
buf dep update
```

コード生成はtonic(prost)にやってもらいたいのでbuf.gen.yamlは今回作成しない。

```bash
# FileDescriptorSetを出力する
buf build -o descriptor.binpb
```

## Rust

```bash
cargo init --bin grpc-web-demo
```

```bash
cargo add tonic prost tonic-prost tonic-reflection
cargo add tokio --features macros,rt-multi-thread
cargo add --build prost tonic-prost-build prost-types
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

## request

```bash
 grpcurl -plaintext -d '{"id": "1234567890"}' localhost:50051 user.v1.UserService.GetUser
{
  "user": {
    "id": "1234567890",
    "name": "John Doe"
  }
}
```
