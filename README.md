# Reproducible build

```
# build
docker run --rm -v $PWD:/build -it ellipsislabs/solana:1.16.16 sh -c "cargo build-bpf -- --locked --frozen"

# get sha and size
sha256sum target/deploy/instruction_forwarder.so
> 44e47408c54cbe1bf58e915595da40eabaf704cf4f9609e583faa9679b4642d5  target/deploy/instruction_forwarder.so
ls -l target/deploy/instruction_forwarder.so
> -rwxr-xr-x 1 root root 51472 Nov 30 08:42 target/deploy/instruction_forwarder.so

# verify (3TS is the executable data for ixF)
solana account 3TSAWmb8f7xe1fG3pgSWroQtL8p28G9nKx56A4cy6FJs -o /tmp/program.bytes
tail -c '+46' /tmp/program.bytes | head -c 51472 | sha256sum
> 44e47408c54cbe1bf58e915595da40eabaf704cf4f9609e583faa9679b4642d5  -
```
