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
   ./target/release/iggy-dashboard-server --host 127.0.0.1 --port 8081
   ```

2. Start the collector (optional, for benchmark result collection):

   ```bash
   cd collector
   # For GitHub polling:
   ./target/release/iggy-dashboard-collector --output-dir /path/to/results poll-github --branch main --interval-seconds 300
   # For local benchmarks:
   ./target/release/iggy-dashboard-collector --output-dir /path/to/results local-benchmark --directory /path/to/iggy --git-ref main --count 5
   ```

3. Access the dashboard at <http://localhost:8081>

### Development Mode

1. Start the backend server:

   ```bash
   cd server
   cargo run  # Will run on port 8081
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
- Backend API server: port 8081

## Configuration

### Server Configuration

The server can be configured using command-line arguments:

```bash
iggy-dashboard-server [OPTIONS]

Options:
    --host <HOST>           Server host [default: 127.0.0.1]
    --port <PORT>           Server port [default: 8081]
    --results-dir <DIR>     Results directory [default: ./performance_results]
    --log-level <LEVEL>     Log level (error|warn|info|debug|trace) [default: info]
    --cors-origins <URLS>   Allowed CORS origins (comma-separated) [default: *]
    --rate-limit <LIMIT>    Rate limit per second [default: 50]
```

### Environment Variables

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

TODO
