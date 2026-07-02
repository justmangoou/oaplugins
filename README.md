# Justmangoou's OpenAction Plugins

### Plugins

| Name                                    | Description                                                                       |
| :-------------------------------------- | :-------------------------------------------------------------------------------- |
| [Macro](crates/macro)                   | A macro plugin for OpenAction                                                     |
| [YTMD Controller](crates/ytmd)          | Control the [YouTube Music Desktop App](https://github.com/ytmdesktop/ytmdesktop) |
| [YTMD Companion](crates/ytmd-companion) | Companion library for OpenAction YTMD Controller                                  |

### Development & Installation

```bash
# Clone the repository
git clone git@github.com:justmangoou/oaplugins.git
cd oaplugins

# Install dependencies
deno install
just check

just build-all # Build all plugins
just build <plugin_name> # Build a specific plugin
```
