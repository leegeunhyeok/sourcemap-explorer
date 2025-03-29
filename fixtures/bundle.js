(() => {
  var __require = /* @__PURE__ */ ((x) =>
    typeof require !== 'undefined'
      ? require
      : typeof Proxy !== 'undefined'
      ? new Proxy(x, {
          get: (a, b) => (typeof require !== 'undefined' ? require : a)[b],
        })
      : x)(function (x) {
    if (typeof require !== 'undefined') return require.apply(this, arguments);
    throw Error('Dynamic require of "' + x + '" is not supported');
  });

  // App.tsx
  var import_react = __require('react');
  var import_react_native2 = __require('react-native');

  // Button.tsx
  var import_react_native = __require('react-native');
  function Button(props) {
    return /* @__PURE__ */ React.createElement(import_react_native.Pressable, {
      ...props,
    });
  }

  // App.tsx
  function App() {
    const [count, setCount] = (0, import_react.useState)(0);
    const handlePress = () => setCount((v) => v + 1);
    return /* @__PURE__ */ React.createElement(
      import_react_native2.View,
      null,
      /* @__PURE__ */ React.createElement(
        import_react_native2.Text,
        null,
        'Count: ',
        count,
      ),
      /* @__PURE__ */ React.createElement(
        Button,
        { onPress: handlePress },
        'Press Me!',
      ),
    );
  }
})();
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsiQXBwLnRzeCIsICJCdXR0b24udHN4Il0sCiAgInNvdXJjZXNDb250ZW50IjogWyJpbXBvcnQgeyB1c2VTdGF0ZSB9IGZyb20gJ3JlYWN0JztcbmltcG9ydCB7IFZpZXcsIFRleHQgfSBmcm9tICdyZWFjdC1uYXRpdmUnO1xuaW1wb3J0IHsgQnV0dG9uIH0gZnJvbSAnLi9CdXR0b24nO1xuXG5leHBvcnQgZnVuY3Rpb24gQXBwKCkge1xuICBjb25zdCBbY291bnQsIHNldENvdW50XSA9IHVzZVN0YXRlKDApO1xuXG4gIGNvbnN0IGhhbmRsZVByZXNzID0gKCkgPT4gc2V0Q291bnQodiA9PiB2ICsgMSk7XG5cbiAgcmV0dXJuIChcbiAgICA8Vmlldz5cbiAgICAgIDxUZXh0PkNvdW50OiB7Y291bnR9PC9UZXh0PlxuICAgICAgPEJ1dHRvbiBvblByZXNzPXtoYW5kbGVQcmVzc30+UHJlc3MgTWUhPC9CdXR0b24+XG4gICAgPC9WaWV3PlxuICApO1xufSIsICJpbXBvcnQgdHlwZSB7IENvbXBvbmVudFByb3BzIH0gZnJvbSAncmVhY3QnO1xuaW1wb3J0IHsgUHJlc3NhYmxlIH0gZnJvbSAncmVhY3QtbmF0aXZlJztcblxuZXhwb3J0IGZ1bmN0aW9uIEJ1dHRvbihwcm9wczogQ29tcG9uZW50UHJvcHM8dHlwZW9mIFByZXNzYWJsZT4pIHtcbiAgcmV0dXJuIDxQcmVzc2FibGUgey4uLnByb3BzfSAvPlxufSJdLAogICJtYXBwaW5ncyI6ICI7Ozs7Ozs7OztBQUFBLHFCQUF5QjtBQUN6QixNQUFBQSx1QkFBMkI7OztBQ0EzQiw0QkFBMEI7QUFFbkIsV0FBUyxPQUFPLE9BQXlDO0FBQzlELFdBQU8sb0NBQUMsaUNBQVcsR0FBRyxPQUFPO0FBQUEsRUFDL0I7OztBRERPLFdBQVMsTUFBTTtBQUNwQixVQUFNLENBQUMsT0FBTyxRQUFRLFFBQUksdUJBQVMsQ0FBQztBQUVwQyxVQUFNLGNBQWMsTUFBTSxTQUFTLE9BQUssSUFBSSxDQUFDO0FBRTdDLFdBQ0Usb0NBQUMsaUNBQ0Msb0NBQUMsaUNBQUssV0FBUSxLQUFNLEdBQ3BCLG9DQUFDLFVBQU8sU0FBUyxlQUFhLFdBQVMsQ0FDekM7QUFBQSxFQUVKOyIsCiAgIm5hbWVzIjogWyJpbXBvcnRfcmVhY3RfbmF0aXZlIl0KfQo=
