# Disco - Discord Bot with HTTP API

A Discord bot built with Rust that also exposes an HTTP API for sending messages.

## Building & Deployment

### Building the Docker Image

**Important**: Use `--no-cache` to ensure you get the latest code:

```bash
# Build without cache
docker build --no-cache -t sweetrose/disco:latest .

# Verify the image works
docker run --rm -e DISCORD_TOKEN="your_token_here" sweetrose/disco:latest
```

### Pushing to Docker Hub

```bash
# Tag for your Docker Hub repository
docker tag sweetrose/disco:latest yourusername/disco:latest

# Push to Docker Hub
docker push yourusername/disco:latest
```

### Deploying to Kubernetes

```bash
# Create the Discord token secret
kubectl create secret generic discord-token --from-literal=token=YOUR_DISCORD_TOKEN_HERE

# Apply the pod configuration
kubectl apply -f disco-pod.yml

# Check status
kubectl get pod disco-pod -w

# View logs
kubectl logs disco-pod -f
```

### Local Development

```bash
# Run locally with cargo
DISCORD_TOKEN="your_token" cargo run

# Or run the debug binary
DISCORD_TOKEN="your_token" ./target/debug/disco

# Or run the release binary
DISCORD_TOKEN="your_token" ./target/release/disco
```

## Environment Variables

- `DISCORD_TOKEN` - Required: Your Discord bot token
- `HTTP_PORT` - Optional: HTTP server port (default: 3000)
- `RUST_LOG` - Optional: Log level (default: info)

## API Endpoints

- `POST /` - Send a message to Discord (multipart form data with `channel_id`, `content`, and optional files)

## Troubleshooting

### Pod exits immediately

Check the logs:
```bash
kubectl logs disco-pod --previous
```

### Image not updating

Make sure to use `--no-cache` when building:
```bash
docker build --no-cache -t sweetrose/disco:latest .
```

### Using local image with MicroK8s

If testing locally with MicroK8s:
```bash
# Import local image
microk8s ctr image import $(docker image inspect sweetrose/disco:latest --format '{{.RepoDigests}}')

# Then apply the pod
microk8s kubectl apply -f disco-pod.yml
```