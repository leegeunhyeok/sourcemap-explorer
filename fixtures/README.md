# fixtures

## Code

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
