# crc32mnemo

Tool providing mnemonic representation for a CRC32 sum over given data

## Installing

```console
$ cargo install crc32mnemo
```

## Using

Generating mnemonic representation out of file data:
```console
$ crc32mnemo /path/to/some/file
```

Generating Bech32 guard out of Bech32 string:
```console
$ crc32mnemo --bech32 bc1qspc5gva3wxqa8ha7tm2pam9qv3k5kptnjeerfp
```
