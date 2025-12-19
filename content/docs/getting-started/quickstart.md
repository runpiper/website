---
title: Quick Start
description: Deploy your first AI agent with RunPiper in under 5 minutes
order: 1
---

# Quick Start Guide

Get your first AI agent running on RunPiper in under 5 minutes.

## Prerequisites

Before you begin, make sure you have:

-   A Unix-like operating system (Linux or macOS)
-   An internet connection
-   Basic familiarity with the command line

> **Windows users**: RunPiper works great with WSL2 (Windows Subsystem for Linux).

## Installation

Install the RunPiper CLI with a single command:

```bash
curl -sSf https://runpiper.ai/install.sh | sh
```

This will download and install the latest version of the RunPiper CLI.

Verify the installation:

```bash
runpiper --version
```

## Create Your First Agent

Let's create a simple "Hello World" agent:

```bash
# Create a new agent project
runpiper init hello-agent

# Navigate to the project directory
cd hello-agent
```

This creates a new directory with a basic agent template.

## Project Structure

Your new agent project looks like this:

```
hello-agent/
├── agent.yaml          # Agent configuration
├── src/
│   └── main.rs        # Your agent code
└── README.md
```

## Configure Your Agent

Open `agent.yaml` and review the configuration:

```yaml
name: hello-agent
version: 1.0.0
runtime: wasm
resources:
    memory: 128MB
    cpu: 0.5
triggers:
    - type: http
      path: /hello
```

## Deploy Your Agent

Deploy to RunPiper Cloud:

```bash
runpiper deploy
```

Or deploy to your self-hosted instance:

```bash
runpiper deploy --target https://your-runpiper-instance.com
```

That's it! Your agent is now live and ready to handle requests.

## Test Your Agent

Once deployed, you'll receive a unique URL for your agent:

```bash
curl https://your-agent-id.runpiper.ai/hello
```

Response:

```json
{
    "message": "Hello from RunPiper!",
    "agent": "hello-agent",
    "version": "1.0.0"
}
```

## Next Steps

Now that you have your first agent running, here's what to explore next:

-   **[Agent Development Guide](/docs/guides/agent-development)**: Learn how to build more complex agents
-   **[Configuration Reference](/docs/reference/configuration)**: Deep dive into agent configuration options
-   **[Deployment Options](/docs/deployment/overview)**: Explore different deployment strategies
-   **[Monitoring & Debugging](/docs/guides/monitoring)**: Learn how to monitor your agents in production

## Getting Help

Need help? We're here for you:

-   📚 Check the [documentation](/docs/getting-started/introduction)
-   💬 Join our [Discord community](https://discord.gg/runpiper)
-   🐛 Report issues on [GitHub](https://github.com/runpiper/runpiper)
-   📧 Email us at support@runpiper.io
