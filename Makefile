CARGO = cargo

CARGO_OPTS =

PEG_GENERATED = src/parser.rs

all:
	$(MAKE) build
	$(MAKE) doc

build: $(PEG_GENERATED)
	$(CARGO) $(CARGO_OPTS) build

clean:
	$(CARGO) $(CARGO_OPTS) clean
	rm $(PEG_GENERATED)

check: $(PEG_GENERATED)
	$(MAKE) build
	$(MAKE) test

test: $(PEG_GENERATED)
	$(CARGO) $(CARGO_OPTS) test

bench: $(PEG_GENERATED)
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

.PHONY: all build clean check test bench doc

$(PEG_GENERATED): peg/sgf.rustpeg
	rust-peg $< > $@
