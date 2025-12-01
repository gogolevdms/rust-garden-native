# solana-native-swaps
Solana Program for Atomic Swaps with SOL

**Prerequisites**

Install [Anchor framework](https://www.anchor-lang.com/docs/installation)

**Getting Started**

1. To build the program:  
```bash
anchor build
```

2. To setup dependencies for tests:
```bash
yarn install
```

3. To run the tests (This compiles the program and deploys it to a built-in test validator):  
```bash
anchor test
```

Use `anchor keys sync` followed by a recompilation to fix any Program ID related issues.