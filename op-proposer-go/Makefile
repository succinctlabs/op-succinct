SHELL := /bin/bash

.PHONY: bindings

bindings:
	@echo "Generating bindings for ZKL2OutputOracle.sol..."
	@mkdir -p bindings
	@cd ../contracts/src/ && forge install && forge build
	@abigen --abi ../contracts/src/ZKL2OutputOracle.sol/ZKL2OutputOracle.abi --bin ../contracts/src/ZKL2OutputOracle.sol/ZKL2OutputOracle.bin --pkg bindings --out bindings/zkl2oo.go
	@echo "Bindings generated successfully."
