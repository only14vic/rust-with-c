-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
libpath = ./$(shell find target -path "*/debug/lib*.so" -exec dirname "{}" \; 2>/dev/null | head -n1)
rustc_sysroot = $(shell rustc --print=sysroot)
rustc_target = $(shell rustc -vV|grep host:|cut -d' ' -f2)

all: vars clean run-std test run test-c check install

run:
	cargo run $(args)

run-std:
	$(make) run args="--no-default-features $(args)"

install: prepare
	cargo install --force --no-track --path . $(args)
	find target -path "*/release/lib*.so" -exec install -D {} lib/ \;

install-std: prepare
	$(make) install args="--no-default-features $(args)"
	install -D $(rustc_sysroot)/lib/rustlib/$(rustc_target)/lib/libstd*.so lib/

check:
	cargo clippy

clean:
	rm -fr target/* bin/* lib/*

prepare:
	mkdir -p bin lib

test:
	find target -path "*/debug/lib*.rlib" -delete
	RUSTFLAGS="-Zpanic_abort_tests -Cpanic=unwind" \
		cargo +nightly test --no-default-features $(args) -- --nocapture --color always

test-c: prepare
	gcc -std=c11 -Os $(args) -Wall -Wno-discarded-qualifiers \
		-Wl,-z,relro,-z,now,-rpath='$$ORIGIN',-rpath='$$ORIGIN/lib',-rpath='$$ORIGIN/../lib',-rpath='$(libpath)' \
		-L$(libpath) -lapp_nostd \
		-o bin/test_lib_c tests/test_lib.c
	./bin/test_lib_c

test-c-gdb:
	$(make) test-c args="-ggdb"

symbols:
	nm -g $(args) \
		$(libpath)/app-nostd \
		$(libpath)/libapp_nostd.so

symbols-dyn:
	$(make) symbols args="-D $(args)"

env:
	env

vars:
	@echo Make variables:
	@echo --------------------------
	@echo rustc_target = $(rustc_target)
	@echo rustc_sysroot = $(rustc_sysroot)
	@echo libpath = $(libpath)
	@echo --------------------------

help:
	@echo -e "\
	Usage guide:\n\n\
	make all		- Build, run, test, check, install\n\
	make install		- Install bins and libs without 'std'\n\
	make install-std	- Install bins and libs with 'std'\n\
	make run		- Compile and run without 'std'\n\
	make run-std		- Compile and run with 'std'\n\
	make check		- Check code\n\
	make clean		- Clean target directory\n\
	make vars		- Show used variables\n\
	make env		- Show used env variables\n\
	make test		- Test Rust code\n\
	make test-c		- Compile and run test C code\n\
	make show-symbols	- Show library symbols\n\
	make show-symbols-dyn	- Show only dynamic library symbols\
	"
