## mimic inscribe with csprs

### install contract

```
casper-client put-deploy --chain-name casper-net-1 \
-n http://localhost:11101/rpc \
--session-path target/wasm32-unknown-unknown/release/cep18.wasm \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-2/secret_key.pem \
--payment-amount 1000000000000
```

### deploy

```
casper-client put-deploy --chain-name casper-net-1 \
-n http://localhost:11101/rpc \
--session-hash e283c070b28250542756463d196807fcfaef2b9670be1f86105b847782984f74 \
--session-entry-point inscribe \
--session-arg "message:string='{\"p\":\"cbrc-20\",\"op\":\"deploy\",\"tick\":\"csprs\",\"max\":\"21000000\",\"lim\":\"1000\",\"decimals\":\"18\"}'" \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-2/secret_key.pem \
--payment-amount 1000000000000
```

### mint

```
casper-client put-deploy --chain-name casper-net-1 \
-n http://localhost:11101/rpc \
--session-hash e283c070b28250542756463d196807fcfaef2b9670be1f86105b847782984f74 \
--session-entry-point inscribe \
--session-arg "message:string='{\"p\":\"cbrc-20\",\"op\":\"mint\",\"tick\":\"csprs\",\"amt\":\"100\"}'" \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-2/secret_key.pem \
--payment-amount 1000000000000
```

### transfer

```
casper-client put-deploy --chain-name casper-net-1 \
-n http://localhost:11101/rpc \
--session-hash e283c070b28250542756463d196807fcfaef2b9670be1f86105b847782984f74 \
--session-entry-point inscribe \
--session-arg "message:string='{\"p\":\"cbrc-20\",\"op\":\"transfer\",\"tick\":\"csprs\",\"amt\":\"2100\",\"to\":\"020226b20c27c32caeda0849a8ff57d6f2836b2f642e8a81f8c3c95080189b8d213f\"}'" \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-2/secret_key.pem \
--payment-amount 1000000000000
```