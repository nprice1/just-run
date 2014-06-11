RUSTC=rustc
RUST_FLAGS=
LDFLAGS=-L lib

.PHONY : all clean doc 

all: clean compile

compile:
	mkdir -p bin
	$(RUSTC) $(RUST_FLAGS) -o bin/just-run $(LDFLAGS) src/main.rs

veyron: RUST_FLAGS += -O -Z time-passes -Z lto
veyron: all


debug: RUST_FLAGS += -g -Z time-passes
debug: compile

deps:	
	cd ../rust-sdl2; git pull; make	
	cp ~/rust-playground/rust-sdl2/build/lib/libsdl2* ../JustRun/lib/
	cp ~/rust-playground/rust-sdl2/build/lib/libsdl2* ../rust-sdl2_mixer/lib/
	cp ~/rust-playground/rust-sdl2/build/lib/libsdl2* ../rust-sdl2_ttf/lib/
	cd ../rust-sdl2_mixer; git pull; rustc -L lib src/sdl2_mixer/lib.rs
	cp ~/rust-playground/rust-sdl2_mixer/libsdl2_mixer* ../JustRun/lib/
	cd ../rust-sdl2_ttf; git pull; rustc -L lib src/sdl2_ttf/lib.rs
	cp ~/rust-playground/rust-sdl2_ttf/libsdl2_ttf* ../JustRun/lib/

doc:
	rustdoc $(LDFLAGS) src/main.rs

run:
	bin/just-run
clean:
	rm -f bin/**
