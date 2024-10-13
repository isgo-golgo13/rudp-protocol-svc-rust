# Variables
PROJECT_NAME := rudp_rust
CLIENT_DIR := ./rudp-socket-client
SERVER_DIR := ./rudp-socket-server
RUDP_PROTOCOL_DIR := ./rudp-protocol-kit
STORAGE_KIT_DIR := ./storage-kit

DOCKER_COMPOSE := docker-compose.yml
CARGO := cargo
IMAGE_TAG := latest

# Commands
DOCKER_COMPOSE_CMD := docker-compose -f $(DOCKER_COMPOSE)
CARGO_BUILD_CMD := $(CARGO) build --release
CARGO_TEST_CMD := $(CARGO) test
CARGO_FMT_CMD := $(CARGO) fmt
CARGO_CLIPPY_CMD := $(CARGO) clippy -- -D warnings

# Phony targets to avoid confusion with actual files
.PHONY: all client server protocol storage build lint fmt test clean docker-build-client docker-build-server up down

# Default target: build and run everything
all: lint test build docker-build up

# Build the client
client:
	@echo "Building the client binary..."
	cd $(CLIENT_DIR) && $(CARGO_BUILD_CMD)

# Build the server
server:
	@echo "Building the server binary..."
	cd $(SERVER_DIR) && $(CARGO_BUILD_CMD)

# Build the protocol library
protocol:
	@echo "Building the RUDP protocol kit..."
	cd $(RUDP_PROTOCOL_DIR) && $(CARGO_BUILD_CMD)

# Build the storage library
storage:
	@echo "Building the storage kit..."
	cd $(STORAGE_KIT_DIR) && $(CARGO_BUILD_CMD)

# Build all binaries (client, server, protocol, storage)
build: client server protocol storage
	@echo "Build complete."

# Lint the Rust code using Clippy
lint:
	@echo "Running Clippy linter..."
	$(CARGO_CLIPPY_CMD)

# Format Rust code
fmt:
	@echo "Formatting Rust code..."
	$(CARGO_FMT_CMD)

# Run unit tests
test:
	@echo "Running tests..."
	$(CARGO_TEST_CMD)

# Clean up build artifacts
clean:
	@echo "Cleaning up binaries and Docker artifacts..."
	rm -rf $(CLIENT_DIR)/target $(SERVER_DIR)/target $(RUDP_PROTOCOL_DIR)/target $(STORAGE_KIT_DIR)/target
	$(DOCKER_COMPOSE_CMD) down --rmi all --volumes --remove-orphans

# Docker build for client
docker-build-client:
	@echo "Building Docker image for client..."
	docker build -t $(PROJECT_NAME)-client:$(IMAGE_TAG) -f $(CLIENT_DIR)/Dockerfile.client .

# Docker build for server
docker-build-server:
	@echo "Building Docker image for server..."
	docker build -t $(PROJECT_NAME)-server:$(IMAGE_TAG) -f $(SERVER_DIR)/Dockerfile.server .

# Build both client and server Docker images
docker-build: docker-build-client docker-build-server
	@echo "Docker build complete."

# Bring up the Docker Compose services
up: docker-build
	@echo "Starting the Docker services..."
	$(DOCKER_COMPOSE_CMD) up --build

# Bring down the Docker Compose services
down:
	@echo "Stopping the Docker services..."
	$(DOCKER_COMPOSE_CMD) down
