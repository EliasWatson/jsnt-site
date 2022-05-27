# JSn't Site
_A static site generator with zero Javascript_

---

## Usage
This project is currently in very early development and isn't really in a usable state.

### Render Example
```shell
jsnt-site examples/basic_site/
# or
cargo run examples/basic_site/
```

## Why no Javascript?
First, let me clarify that I do not hate Javascript.
It's probably one of the most influential contributions to the internet.
However, it is massively overused.
An interactive web app needs Javascript.
A simple blog does not.

### Benefits of not using JS
- No trackers
- More secure
- No automated ads
- Very small page sizes _(ignoring images/videos/audio)_
- Performant on very low-spec systems
- Usable with terminal web browsers
- Minimalist

## Features
### Planned
- Custom templates
- Write page content in Markdown
- Insert data from external files
  - For instance, insert a CSV file as a table
- Use custom HTML in Markdown
- Automatic image compression
- Optionally convert images to ASCII art
- Pre-rendered code syntax highlighting
- Evaluate codeblocks or scripts and include output in page
- Basic scripting in Lua for automating page content
- Output to alternative web protocols such as Gopher and Gemini
