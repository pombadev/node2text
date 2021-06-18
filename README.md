# node2text
> A tool to extract text from HTML from your terminal.

# Usage

```bash
# pipe in
curl -s 'https://en.wikipedia.org/wiki/Wiki' | node2text '#siteSub'
# Outputs: From Wikipedia, the free encyclopedia

# extract from path
node2text '#app.title' /path/to/file.html
# May or may not output depending on if selector is matched
```

# Motivation
When I reinstall my machine, I want to automate my install process. Usually it involves quickly grabbing snippet from the internet and writing it to file, this tool aims to help script it.

Hugely inspired by [pup](https://github.com/ericchiang/pup).

# Demo

[![demo](./assets/demo.svg)](./assets/demo.svg)
