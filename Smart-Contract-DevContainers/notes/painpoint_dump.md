## Pain Points

- Feels bad

![Paseo address issue](../assets/issues1.png)

- Nice to have details— but where do I put these?
  - might want websocket url

![Paseo address issue](../assets/issues2.png)

- I need to generating a paseo address to work with (non-obvious how to do, I'd prefer not to have to navigate a UI as well)
- I decided I want to use `npx @polkadot/api-cli` to generate an account, monitor it, and do whatever other chain commmands I'll have to do. However commands against both of the following seem to hang w/o any feedback

```
PASEO1=wss://paseo.rpc.ibp.network
PASEO2=wss://paseo-rpc.polkadot.io
```

    - I found `wss://paseo-rpc.dwellir.com` which seems to work
