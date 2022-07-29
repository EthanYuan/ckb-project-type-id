TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
AR := $(TARGET)-ar
OBJCOPY := $(TARGET)-objcopy
CFLAGS := -fPIC -O3 -fno-builtin-printf -fno-builtin-memcmp -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -fdata-sections -ffunction-sections -I ckb-c-std-lib -I ckb-c-std-lib/libc -I ckb-c-std-lib/molecule -I ckb-c-std-lib/simulator -I c -I build -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -g

SCRIPT_CFLAGS := -I ckb-c-std-lib -I ckb-c-std-lib/libc -I ckb-c-std-lib/molecule -I ckb-c-std-lib/simulator

BUILDER_DOCKER := jjy0/ckb-capsule-recipe-rust:2020-9-28

all: build/libckb-lib-type-id.a

all-via-docker: 
	docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && make all"

build/libckb-lib-type-id.a: build/type_id.o
	$(AR) rcs $@

build/type_id.o: type_id.c

clean:
	rm -f build/*.o build/*.a

clean-x86:
	rm -f build-x86/*.o build-x86/*.a

