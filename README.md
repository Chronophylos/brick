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