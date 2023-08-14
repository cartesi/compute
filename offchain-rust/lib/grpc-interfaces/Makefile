# Build settings
DOCDIR := $(abspath doc)

all: doc

clean:
	rm -rf $(DOCDIR)

doc:
	docker run --rm -v `pwd`/doc:/out -v `pwd`:/protos pseudomuto/protoc-gen-doc

.PHONY: all doc clean
