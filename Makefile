-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
libpath = $(shell find target -type d -name debug|head -n1)
rustc_sysroot = $(shell rustc --print=sysroot)
rustc_target = $(shell rustc -vV|grep host:|cut -d' ' -f2)

all: vars clean check run
	$(make) clean test
	$(make) clean run-no-std test-c
	$(make) install-no-std

run:
	cargo run $(args)

run-no-std:
	$(make) run args="--no-default-features $(args)"

install: prepare
	cargo install --force --no-track --path . $(args)
	find target -path "*/release/lib*.so" -exec install -D {} lib/ \;
	install -D $(rustc_sysroot)/lib/rustlib/$(rustc_target)/lib/libstd*.so lib/

install-no-std: prepare
	$(make) install args="--no-default-features $(args)"

check:
	cargo check --no-default-features
	cargo check
	cargo clippy --no-deps
	rustup run nightly rustfmt --check src/**

prepare:
	mkdir -p bin lib

clean:
	find ./target ./bin ./lib \
		   -path "./bin/*" -delete \
		-o -path "./lib/*" -delete \
		-o -path "./target/*" -a -name "*app*" \
			-type f -executable -delete

test:
	RUSTFLAGS="-Zpanic_abort_tests -Cpanic=unwind" \
		cargo +nightly test $(args) -- --nocapture --color always

test-c: prepare
	# Strip debuginfo and symbols: -g -s
	cc -std=gnu18 -Os -pthread $(args) -Wall -Wextra \
		-Wl,-z,relro,-z,now,-rpath='$$ORIGIN',-rpath='$$ORIGIN/lib',-rpath='$$ORIGIN/../lib',-rpath='$$ORIGIN/../$(libpath)' \
		-L$(libpath) -lapp_nostd -ljson-c -linih \
		-o bin/test_lib_c tests/test_lib.c
	./bin/test_lib_c

test-c-gdb:
	$(make) test-c args="-ggdb"

symbols:
	nm -g $(args) \
		bin/test_lib_c \
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

_git-pre-commit: check

_git-pre-push: all

help:
	@echo -e "\
	Usage guide:\n\n\
	make all		- Build, run, test, check, install \n\
	make install		- Install bins, libs with 'std' lib \n\
	make install-no-std	- Install bins, libs without 'std' lib \n\
	make run		- Compile and run with 'std' lib \n\
	make run-no-std		- Compile and run without 'std' lib \n\
	make check		- Check code \n\
	make clean		- Clean target directory \n\
	make vars		- Show used variables \n\
	make env		- Show used env variables \n\
	make test		- Test Rust code \n\
	make test-c		- Compile and run test C code \n\
	make show-symbols	- Show library symbols \n\
	make show-symbols-dyn	- Show only dynamic library symbols\
	"
