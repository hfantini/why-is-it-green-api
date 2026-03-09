# AGENTS.md

## Project overview

This repository contains the **Rust API** for the `why-is-it-green` project.

The project is part of a **DevOps and infrastructure learning lab** where the goal is to experiment with modern deployment pipelines, containerization, and cloud platforms.

The project currently consists of:

* a **Vue frontend application**
* a **Rust backend API**
* container-based deployment
* CI/CD pipelines
* cloud deployment on Render

The frontend is already functional and deployed to production.
This repository implements the backend API that supports it.

The primary goal of this project is **learning and experimentation**, not production-grade scale.

---

# Technology stack

Backend language

Rust (edition 2024)

Web framework

Axum

Async runtime

Tokio

Serialization

Serde

Tooling

cargo
rustfmt
clippy

---

# DevOps context

This project is designed to explore real-world DevOps workflows.

Topics being practiced include:

* containerized builds
* CI pipelines
* CD pipelines
* staging and production environments
* cloud deployment
* infrastructure automation

The backend should therefore remain **container-friendly and stateless when possible**.

---

# Existing frontend

The project already contains a working frontend built with:

Vue + Vite + TypeScript

The frontend pipeline already includes:

* CI checks
* Docker image build
* deployment pipeline
* staging environment
* production environment

The Rust API will eventually be integrated into the same deployment workflow.

---

# Development commands

Build

cargo build

Run

cargo run

Format

cargo fmt

Lint

cargo clippy

Strict lint

cargo clippy -- -D warnings

---

# Architecture

The backend follows a **simplified Hexagonal Architecture (Ports & Adapters)**.

Layers

adapters → services → domain

Rules

* dependencies always point inward
* domain must not depend on infrastructure
* adapters must not contain business logic
* services orchestrate domain logic

---

# Folder structure

src

adapters
services
domain
ports

main.rs

Explanation

domain
Domain entities and core business concepts.

services
Application services responsible for orchestrating domain logic.

ports
Traits defining contracts between services and infrastructure.

adapters
Infrastructure implementations.

---

# Inbound adapters

Inbound adapters represent **entry points into the system**.

Examples

* HTTP handlers
* CLI commands
* message consumers

Currently only **HTTP via Axum** is implemented.

Handlers live in

adapters/inbound/http

Handlers should remain thin and only translate HTTP requests into service calls.

---

# Outbound adapters

Outbound adapters implement external integrations.

Examples

* JSON repositories
* databases
* external APIs
* caches

These adapters implement traits defined in the `ports` module.

---

# Persistence strategy

At the moment the backend **does not use a database**.

Data may be stored in simple JSON files.

This decision keeps infrastructure simple while the goal is to explore backend architecture and DevOps workflows.

A real database may be introduced in a future iteration.

---

# HTTP endpoints

Example endpoints

/health
Used for infrastructure health checks.

/why-is-it-green
Returns the reason why the system is currently considered "green".

Health endpoints should remain infrastructure-only and should not interact with domain logic.

---

# Code style

Follow idiomatic Rust.

Requirements

* code must pass rustfmt
* code must pass clippy
* avoid unnecessary cloning
* avoid unwrap in non-test code
* prefer Result-based error handling

---

# Module conventions

Rust modules may use `mod.rs` when directories represent modules.

Example

adapters
mod.rs

inbound
mod.rs

http
mod.rs

health_handler.rs

Use `pub mod` to expose modules.

---

# Design goals

This repository prioritizes

* architectural clarity
* low coupling
* testability
* container-friendly design
* DevOps experimentation

Avoid unnecessary complexity.

---

# Agent guidelines

When editing this repository

* respect the hexagonal architecture
* keep HTTP handlers thin
* avoid direct repository usage inside handlers
* services should orchestrate domain logic
* prefer small focused modules

Changes should remain consistent with the project's learning goals and DevOps experimentation context.
