(() => {
  // index.js
  function main() {
    print("Hello, world!");
    throw new Error("Boom!");
  }
  typeof print === "undefined" && (globalThis.print = console.log);
  main();
})();
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsiaW5kZXguanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImZ1bmN0aW9uIG1haW4oKSB7XG4gIHByaW50KCdIZWxsbywgd29ybGQhJyk7XG5cbiAgdGhyb3cgbmV3IEVycm9yKCdCb29tIScpO1xufVxuXG50eXBlb2YgcHJpbnQgPT09ICd1bmRlZmluZWQnICYmIChnbG9iYWxUaGlzLnByaW50ID0gY29uc29sZS5sb2cpXG5cbm1haW4oKTsiXSwKICAibWFwcGluZ3MiOiAiOztBQUFBLFdBQVMsT0FBTztBQUNkLFVBQU0sZUFBZTtBQUVyQixVQUFNLElBQUksTUFBTSxPQUFPO0FBQUEsRUFDekI7QUFFQSxTQUFPLFVBQVUsZ0JBQWdCLFdBQVcsUUFBUSxRQUFRO0FBRTVELE9BQUs7IiwKICAibmFtZXMiOiBbXQp9Cg==
