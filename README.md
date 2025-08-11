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

```sh
trunk serve --open
```

Open [http://localhost:8080/control-box](http://localhost:8080/control-box) in your browser.

### Project Structure

- `simulator/src/components/` — Yew components (signal, range, plot, dialogs)
- `simulator/src/pages/` — Application pages
- `control-box/src/signal/` — Signal trait and implementations
- `build.rs` — Embeds git and version info into the build

### Development

- Edit Rust/Yew code in `simulator/src/`
- Add new signal types in `control-box/src/signal/`
- Use Trunk for hot-reload and WASM builds

## License

MIT


---

**Note:**  
This project uses [FontAwesome](https://fontawesome.com/) for icons and [Plotly](https://plotly.com/javascript/) for plotting.  
See `index.html` for CDN links.