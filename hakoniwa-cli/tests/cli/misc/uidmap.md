# --uidmap

Custom UID in the container

## mapping UID

```console
$ hakoniwa run --uidmap 0 -- id
uid=0(root) gid=1[..]

```

## cli arg name `-u`

```console
$ hakoniwa run -u 0 -- id
uid=0(root) gid=1[..]

```
