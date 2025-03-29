# fixtures

## Code

ESBuild [Playground](https://esbuild.github.io/try/#YgAwLjI1LjEAeyBzb3VyY2VtYXA6IHRydWUsIGJ1bmRsZTogdHJ1ZSwgZXh0ZXJuYWw6IFsicmVhY3QiLCAicmVhY3QtbmF0aXZlIl0gfQBlAGluZGV4LmpzAGZ1bmN0aW9uIG1haW4oKSB7CiAgcHJpbnQoJ0hlbGxvLCB3b3JsZCEnKTsKCiAgdGhyb3cgbmV3IEVycm9yKCdCb29tIScpOwp9Cgp0eXBlb2YgcHJpbnQgPT09ICd1bmRlZmluZWQnICYmIChnbG9iYWxUaGlzLnByaW50ID0gY29uc29sZS5sb2cpCgptYWluKCk7)

```tsx
// index.js
function main() {
  print('Hello, world!');

  throw new Error('Boom!');
}

typeof print === 'undefined' && (globalThis.print = console.log)

main();
```

## Hermes

### Compile HBC

Build [https://github.com/facebook/hermes](hermes) and run `hermesc` to compile the code into a HBC file.

```sh
hermesc bundle.js -emit-binary -out bundle.hbc -output-source-map
```

### Run

```sh
hermes bundle.hbc
```
