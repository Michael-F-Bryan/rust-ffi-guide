CRATES := "magic" "magic-sys"

define invoke_cargo
	for crate in $(CRATES); do \
		cargo $(1) --manifest-path $$crate/Cargo.toml; \
	done
endef

test:
	$(call invoke_cargo,test)

build:
	$(call invoke_cargo,build)

clean:
	$(call invoke_cargo,clean)

.PHONY: test build clean