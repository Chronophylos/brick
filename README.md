**Work in Progress**

<hr>

# Brick - A supercharged archiving tool

## Features

- Compress and decompress data from different formats
- Support nested archives (eg. tar in gzip)
- Multithreaded

### Supported formats

- lzma
- gzip
- bzip
- bzip2
- zip
- rar
- tar
- bzip
- zlib / zstandard


## Examples

### Unpack a file

```
brick unpack archive.zip
brick u archive.zip
```

### Pack a file

```
brick pack directory archive.zip
brick p directory archive.zip
```

### Pack with specific format

```
brick p -f zip directory archive
```

If you leave the archive file name out brick will use `directory[.ext]+`.

```
brick p -f zip directory
```

Creates `directory.zip`.

### Pack with multiple formats

```
brick p -f zip p -f rar directory
```

Creates `directory.zip.rar`

## Related Work

- [ouch](https://github.com/ouch-org/ouch) - Painless compression and decompression for your terminal

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>