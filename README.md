# ckb-project-type-id

### Pre-requirement

- [capsule](https://github.com/nervosnetwork/capsule) >= 0.4.3
- [ckb-cli](https://github.com/nervosnetwork/ckb-cli) >= 0.35.0

> Note: Capsule uses docker to build contracts and run tests. https://docs.docker.com/get-docker/
> and docker and ckb-cli must be accessible in the PATH in order for them to be used by Capsule.

### Getting Started

- Init submodules:

```
git submodule init && git submodule update -r --init
```

- Generator static linking for ckb std lib:

```
cd contracts/ckb-project-type-id/ckb-c-stdlib/ckb-c-stdlib
git submodule init && git submodule update
cd .. && make all-via-docker
```


- Build contracts:

``` sh
# back to repo root directory
cd ../../..
capsule build
```

- Run tests:

``` sh
capsule test
```
