# Large state service

A Restate service that generates large state entries. You call it via:

```
curl localhost:8080/LargeState/1/state --json '<state-size-in-bytes>'
```

This will create a state entry of random bytes of the specified size.
