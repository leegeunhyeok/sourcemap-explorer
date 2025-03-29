# fixtures

## Code

ESBuild [Playground](https://esbuild.github.io/try/#YgAwLjI1LjEAeyBzb3VyY2VtYXA6IHRydWUsIGJ1bmRsZTogdHJ1ZSwgZXh0ZXJuYWw6IFsicmVhY3QiLCAicmVhY3QtbmF0aXZlIl0gfQBlAEFwcC50c3gAaW1wb3J0IHsgdXNlU3RhdGUgfSBmcm9tICdyZWFjdCc7CmltcG9ydCB7IFZpZXcsIFRleHQgfSBmcm9tICdyZWFjdC1uYXRpdmUnOwppbXBvcnQgeyBCdXR0b24gfSBmcm9tICcuL0J1dHRvbic7CgpleHBvcnQgZnVuY3Rpb24gQXBwKCkgewogIGNvbnN0IFtjb3VudCwgc2V0Q291bnRdID0gdXNlU3RhdGUoMCk7CgogIGNvbnN0IGhhbmRsZVByZXNzID0gKCkgPT4gc2V0Q291bnQodiA9PiB2ICsgMSk7CgogIHJldHVybiAoCiAgICA8Vmlldz4KICAgICAgPFRleHQ+Q291bnQ6IHtjb3VudH08L1RleHQ+CiAgICAgIDxCdXR0b24gb25QcmVzcz17aGFuZGxlUHJlc3N9PlByZXNzIE1lITwvQnV0dG9uPgogICAgPC9WaWV3PgogICk7Cn0AAEJ1dHRvbi50c3gAaW1wb3J0IHR5cGUgeyBDb21wb25lbnRQcm9wcyB9IGZyb20gJ3JlYWN0JzsKaW1wb3J0IHsgUHJlc3NhYmxlIH0gZnJvbSAncmVhY3QtbmF0aXZlJzsKCmV4cG9ydCBmdW5jdGlvbiBCdXR0b24ocHJvcHM6IENvbXBvbmVudFByb3BzPHR5cGVvZiBQcmVzc2FibGU+KSB7CiAgcmV0dXJuIDxQcmVzc2FibGUgey4uLnByb3BzfSAvPgp9)

```tsx
// App.tsx
import { useState } from 'react';
import { View, Text } from 'react-native';
import { Button } from './Button';

export function App() {
  const [count, setCount] = useState(0);

  const handlePress = () => setCount((v) => v + 1);

  return (
    <View>
      <Text>Count: {count}</Text>
      <Button onPress={handlePress}>Press Me!</Button>
    </View>
  );
}
```

```tsx
// Button.tsx
import { Pressable } from 'react-native';

export function Button(props: ComponentProps<typeof Pressable>) {
  return <Pressable {...props} />;
}
```

## Compile HBC

Build [https://github.com/facebook/hermes](hermes) and run `hermesc` to compile the code into a HBC file.

```sh
hermesc input.js -emit-binary -out output.hbc -output-source-map
```
