<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">Plausible component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/plausible-component/badge.svg)](https://coveralls.io/github/edgee-cloud/plausible-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/plausible-component.svg)](https://github.com/edgee-cloud/plausible-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/plausible)


This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [Plausible](https://plausible.io), allowing you to collect and forward analytics events to your Plausible instance.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `plausible.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.data_collection]]
id = "plausible"
file = "/var/edgee/components/plausible.wasm"
# settings.instance_url = "https://plausible.io"
settings.domain = "YOUR_SITE_DOMAIN"
```


## Event Handling

### Event Mapping
The component maps Edgee events to plausible Event as follows:

| Edgee Event | plausible Event | Description |
|-------------|----------------|-------------|
| Page        | `pageview` | pageview event with url as property |
| Track       | Your event name | A event with your custom event name |
| User        | `user` | Sets all the data for the user |


## Configuration Options

### Basic Configuration
```toml
[[destinations.data_collection]]
id = "plausible"
file = "/var/edgee/components/plausible.wasm"
# settings.instance_url = "https://plausible.io"
settings.domain = "YOUR_SITE_DOMAIN"
```


### Event Controls
Control which events are forwarded to plausible:
```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```


## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- [Edgee CLI](https://github.com/edgee-cloud/edgee)

Build command:
```bash
edgee components build
```

Test commands:
```bash
edgee components test
cargo test
```

Test coverage command:
```bash
cargo llvm-cov --all-features
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
