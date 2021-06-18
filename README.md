# node2text
> A tool tool to parse HTML in your terminal.

# Usage

```bash
# pipe in
curl -s 'https://en.wikipedia.org/wiki/Wiki' | node2text '#siteSub' # From Wikipedia, the free encyclopedia

# extract from path
node2text '#app.title' /path/to/file.html # some text
```
