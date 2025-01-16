# Iggy Dashboard

A modern, high-performance benchmark results dashboard for Iggy, built with Rust. This application provides a responsive web interface for visualizing and analyzing benchmark results.

## Features

- 📊 Interactive performance trend visualization
- 🔍 Filter benchmarks by hardware and version
- 📱 Responsive design that works on desktop and mobile
- 🚀 High-performance Rust backend
- ⚡ Fast, modern web frontend built with Yew
- 🔒 Built-in rate limiting and CORS protection

## Project Structure

The project is organized as a Rust workspace with four main components:

- `frontend/`: Yew-based web application
  - Modern UI with interactive charts
  - Type-safe API integration
  - Error handling and loading states

- `server/`: Actix-web REST API server
  - Efficient file system operations
  - Configurable through command-line arguments
  - Built-in security features

- `collector/`: Benchmark results collector
  - Polls GitHub for CI benchmark results
  - Runs local benchmarks and collects results
  - Organizes benchmark results in a structured format
  - Supports both local and remote result collection

- `shared/`: Common code between components
  - Type definitions
  - Serialization logic
  - Shared constants

## Prerequisites

- Rust toolchain (latest stable)

  ```bash
  rustup default stable
  ```

- WebAssembly target

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- Trunk (for frontend development)

  ```bash
  cargo install trunk
  ```

## Development Setup

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd iggy-yew
   ```

2. Build the frontend:

   ```bash
   cd frontend
   trunk build --release
   ```

3. Build the server:

   ```bash
   cd ../server
   cargo build --release
   ```

## Running the Application

### Production Mode

1. Start the server:

   ```bash
   cd server
   ./target/release/iggy-dashboard-server --host 127.0.0.1 --port 8061
   ```

2. Start the collector (optional, for benchmark result collection):

   ```bash
   cd collector
   # For GitHub polling:
   ./target/release/iggy-dashboard-collector --output-dir /path/to/results poll-github --branch main --interval-seconds 300
   # For local benchmarks:
   ./target/release/iggy-dashboard-collector --output-dir /path/to/results local-benchmark --directory /path/to/iggy --git-ref main --count 5
   ```

3. Access the dashboard at <http://localhost:8061>

### Development Mode

1. Start the backend server:

   ```bash
   cd server
   cargo run  # Will run on port 8061
   ```

2. Start the collector (optional, for benchmark result collection):

   ```bash
   cd collector
   # For GitHub polling:
   cargo run -- --output-dir /path/to/results poll-github --branch main --interval-seconds 300
   # For local benchmarks:
   cargo run -- --output-dir /path/to/results local-benchmark --directory /path/to/iggy --git-ref main --count 5
   ```

3. In a separate terminal, start the frontend development server:

   ```bash
   cd frontend
   trunk serve  # Will run on port 8080
   ```

4. Access the development version at <http://localhost:8080>

Note: The development setup uses different ports:

- Frontend development server: port 8080
- Backend API server: port 8061

## Running with Docker

### Building the Image

```bash
docker build -t iggy-dashboard .
```

### Running the Container

1. First, ensure your performance results directory exists and has proper permissions:

   ```bash
   mkdir -p performance_results
   chmod 755 performance_results
   ```

2. Run the container:

Basic usage (recommended):

   ```bash
   docker run -p 8061:8061 \
      -v "$(pwd)/performance_results:/data/performance_results" \
      --user "$(id -u):$(id -g)" \
      iggy-dashboard
   ```

With custom configuration:

   ```bash
   docker run -p 8061:8061 \
      -v "$(pwd)/performance_results:/data/performance_results" \
      --user "$(id -u):$(id -g)" \
      -e HOST=0.0.0.0 \
      -e PORT=8061 \
      -e RESULTS_DIR=/data/performance_results \
      iggy-dashboard
   ```

Using a named volume:

   ```bash
   # Create a named volume
   docker volume create iggy-results

   # Run with named volume
   docker run -p 8061:8061 \
      -v iggy-results:/data/performance_results \
      iggy-dashboard
   ```

## Configuration

### Docker Configuration

#### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| HOST | 0.0.0.0 | Server host address |
| PORT | 8061 | Server port |
| RESULTS_DIR | /data/performance_results | Directory for performance results |

#### Volume Permissions

The container is configured to run as a non-root user for security. When mounting a local directory, you should:

1. Use the `--user` flag with your local user ID to ensure proper file permissions
2. Make sure your local directory has the correct permissions (755)
3. If using a named volume, the container will handle permissions automatically

### Application Configuration

#### Server Settings

The server can be configured using command-line arguments:

```bash
iggy-dashboard-server [OPTIONS]

Options:
      --host <HOST>                  Server host address [default: 127.0.0.1]
      --port <PORT>                  Server port [default: 8061]
      --results-dir <RESULTS_DIR>    Directory containing performance results [default: ./performance_results]
      --log-level <LOG_LEVEL>        Log level (trace, debug, info, warn, error) [default: info]
      --cors-origins <CORS_ORIGINS>  Allowed CORS origins (comma-separated) [default: *]
      --rate-limit <RATE_LIMIT>      Rate limit per second per IP [default: 500]
  -h, --help                         Print help
  -V, --version                      Print version
```

### Environment Variables for Development

For development, you can also use environment variables:

- `RUST_LOG`: Control log level and filters
- `RUST_BACKTRACE`: Enable backtraces (1 = enabled, full = full backtraces)

## API Endpoints

The server provides the following REST API endpoints:

- `GET /api/hardware` - List available hardware configurations
- `GET /api/versions/{hardware}` - List versions for specific hardware
- `GET /api/benchmarks/{version}` - List benchmarks for a version
- `GET /api/benchmarks/{version}/{hardware}` - List benchmarks for version and hardware
- `GET /api/benchmark_info/{path}` - Get detailed benchmark information
- `GET /health` - Server health check

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Apache-2.0
