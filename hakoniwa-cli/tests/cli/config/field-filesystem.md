# CfgFileSystem

## file

```console
$ hakoniwa run --config ./tests/fixtures/config/field-filesystem.toml -- cat /myfile
abc
```

## dir

```console
$ hakoniwa run --config ./tests/fixtures/config/field-filesystem.toml -- stat --printf %A /mydir
drwx------
```
