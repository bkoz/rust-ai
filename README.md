# rust-ai
Rust examples that makes inference against models hosted by Ollama and Llamastack.  

This code expects:
- A working RUST installation.
- The Ollama model server hosting an LLM like `llama3.1:8b`
- A Llamastack server (for the llamastack example).

```bash
export LLAMA_STACK_MODEL="llama3.1:8b"
export INFERENCE_MODEL="llama3.1:8b"
export LLAMA_STACK_PORT=8321
export LLAMA_STACK_SERVER=http://localhost:$LLAMA_STACK_PORT
```

Use `podman` or `docker` to run the llamastack server.

```bash
podman run --name=llamastack --network=host \                   
  -d -p $LLAMA_STACK_PORT:$LLAMA_STACK_PORT \
  llamastack/distribution-ollama:0.2.9 \
  --port $LLAMA_STACK_PORT \
  --env INFERENCE_MODEL=$LLAMA_STACK_MODEL \
  --env OLLAMA_URL=http://localhost:11434
  ```

