<div align="center">

# harview

HTTP Archive (HAR) Viewer on the terminal written in Rust

*This tool is still under development. Please note that specifications are subject to change without notice.*

</div>


## About

**harview** is a viewer of HAR files that runs on the terminal. You can easily view HAR files exported from a web browser without opening the browser.

The goal of this tool is not to provide in-depth analysis capabilities like DevTool, but to provide the ability to browse HAR files with a lightweight UI like an easy-to-use pager for those familiar with the command line interface.

## Usage

### Export HAR files from Browsers

Open your web browser's DevTools and export the HAR file.
It supports the HAR format, which can be exported in the following browsers:

- Firefox
- Chromium based browsers

**Firefox**:
    
1. Open DevTool (type `F12` or `Ctrl-Shift-I`)
2. Open *Network* tab and select *Save All as HAR* from context menu

> [Network request list - Firefox Source Docs](https://firefox-source-docs.mozilla.org/devtools-user/network_monitor/request_list/index.html)

**Chromium**:

1. Open DevTool (type `F12` or `Ctrl-Shift-I`)
2. Click *Export HAR* below the tab bar

> [Network features reference - Chrome DevTools](https://developer.chrome.com/docs/devtools/network/reference)

### Use *harview* TUI

To use harview, specify the path of the HAR file as the first argument. Once the HAR file is loaded, an entry in the HTTP log will appear in the table.

```sh
harview example.com.har
```
TUI can be controlled with the following keys:

| Key | Action |
|-----|---------|
| `k` / `j` | Move the focus up / down |
| `u` /  `d` | Move the focus up / down more fast |
| `1` - `4` | Switch preview widget tab |
| `q` or `Ctrl-C` | Quit application |

## Installation

Clone this repository then run `cargo install`

```sh
git clone https://github.com/sheepla/harview.git
cd harview
cargo install --path .
```

## References

- [HAR (file format) - Wikipedia](https://en.wikipedia.org/wiki/HAR_%28file_format%29)
- [HTTP Archive (HAR) Format Specifications - W3C](https://w3c.github.io/web-performance/specs/HAR/Overview.html)

## Thanks

- [ratatui](https://ratatui.rs/) - This tool was built with ratatui, a TUI library for Rust. Thank you for the amazing library and its ecosystem!
