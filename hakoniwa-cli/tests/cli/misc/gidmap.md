# --gidmap

Custom GID in the container

## mapping GID

```console
$ hakoniwa run --gidmap 0 -- id
uid=1[..] gid=0(root) [..]

```

## cli arg name `-g`

```console
$ hakoniwa run -g 0 -- id
uid=1[..] gid=0(root) [..]

```
