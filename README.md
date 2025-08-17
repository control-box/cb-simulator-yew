# Control Box Simulator

A Rust/Yew web application for simulating and visualizing time-domain signals.

## Features

- Interactive signal creation and editing (step, noise, superposition)
- Time range configuration
- Plotly-based signal visualization
- Modular component architecture
- Git commit/tag/version info embedded at build time

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org) (edition 2021)
- [Trunk](https://trunkrs.dev/) for building and serving WASM web apps
- [Node.js](https://nodejs.org/) (for some frontend tooling, optional)

### Build & Run

1. **Install dependencies:**
    ```bash
    cargo install trunk
    npm install
    ```

1. **Create tailwindcss output:**
    ```bash
    npm run build:css
    ```


2. **Run the development server:**
    ```bash
    trunk serve --open
    ```

3. **Build for production:**
    ```bash
    trunk build --release
    ```


Open [http://localhost:8080/control-box](http://localhost:8080/cb-simulator-yew) in your browser.

### Project Structure


- `build.rs` — Embeds git and version info into the build

### Development



## License

MIT — see [`LICENSE.md`](LICENSE.md)

---

**Note:**  
This project uses [FontAwesome](https://fontawesome.com/) for icons and [Plotly](https://plotly.com/javascript/) for plotting.  
See `index.html` for CDN links.