---
title: Introduction to RunPiper
description: Learn what RunPiper is and how it can help you deploy AI agents at scale
---

# Introduction to RunPiper

RunPiper is an **open-source, enterprise-grade AI agent runtime** built in Rust. It's designed to solve the hard problems of running AI agents reliably at scale.

## What is RunPiper?

RunPiper provides a production-ready runtime environment for deploying and managing thousands of concurrent AI agents. Think of it as the infrastructure layer that sits between your agent code and the hardware it runs on.

## Key Features

### Memory-Safe Execution

Built in Rust, RunPiper guarantees memory safety and thread safety at compile time. This eliminates entire classes of bugs that plague other runtimes.

### Sub-Millisecond Cold Starts

Deploy agents instantly with cold start times under 1 millisecond. No more waiting for containers to spin up or interpreters to initialize.

### Framework Agnostic

RunPiper works with any agent framework:

- LangChain
- AutoGPT
- Custom frameworks
- Any language that compiles to WebAssembly

### Cloud or Self-Hosted

Choose your deployment model:

- **RunPiper Cloud**: One-click deploy with zero infrastructure management
- **Self-Hosted**: Full control over your infrastructure

## Architecture Overview

RunPiper consists of three main components:

1. **Control Plane**: Manages agent lifecycle and orchestration
2. **Runtime**: Executes agent code in isolated sandboxes
3. **Storage Layer**: Handles persistent state and memory

```
┌─────────────────────────────────────┐
│         Control Plane               │
│  (Orchestration & Management)       │
└──────────────┬──────────────────────┘
               │
┌──────────────┴──────────────────────┐
│         Runtime Layer                │
│   (Agent Execution Sandboxes)       │
└──────────────┬──────────────────────┘
               │
┌──────────────┴──────────────────────┐
│        Storage Layer                 │
│  (State & Memory Management)        │
└─────────────────────────────────────┘
```

## Use Cases

RunPiper is perfect for:

- **Customer Support Agents**: Handle thousands of concurrent customer conversations
- **Data Processing Pipelines**: Run agents that process and transform data at scale
- **Autonomous Systems**: Deploy self-directed agents that operate independently
- **Research & Development**: Experiment with agent architectures in a production environment

## Next Steps

Ready to get started? Check out our [Quick Start Guide](/docs/getting-started/quickstart) to deploy your first agent in under 5 minutes.

Or dive deeper into the [Architecture Documentation](/docs/architecture/overview) to understand how RunPiper works under the hood.

