Key Manager is a utility to make centralized SSH key management a bit less tedious. It provides several utilities for
managing common public keys, as well as a utility for compiling known public keys into a single sshd-compatible
`authorized_keys` file.

It is intended to be used alongside some method of distributing the compiled key file (e.g. via a private Nginx server),
which could then be placed on any remote systems that should be accessible by users that own the managed keys.

# Usage
After installing the tool, the first thing you'll want to do is add a key. Public keys are stored in groups, usually
named after the person or entity that owns the key (e.g. "tyler" or "gravitational-marketing"). A group can have any
number of keys; however, each key must have a unique name.

Adding a new key is simple.

```shell
$ key-manager add my-group "ssh-rsa AAA ..."
```

An additional argument can be passed to the `add` subcommand to specify the name of the key file. If not provided, the
value of comment section of the public key will be used as the key's name.

Once you've added your keys, the following command can be used to compile all known keys into a single `authorized_keys`
file.

```shell
$ key-manager compile /var/www/html/authorized_keys
```

By default, the `compile` subcommand will not do anything if no keys have been added since it was last run. To force
the output file to be regenerated, you should pass the `--force` flag.

Some other useful commands can be found below.

```shell
$ key-manager list
global (1 key(s))
  - MyGlobalKey

tyler (3 key(s))
  - tyler@zodiark
  - tyler@hydaelyn
  - tyler@quetzelcoatl

$ key-manager remove global MyGlobalKey
Deleted 1 key named MyGlobalKey
$ key-manager remove tyler
Deleted 3 key(s)
```
