# TrustedChain Linera application

## Usage

You can install and use Linera application on testnet and local network.

### Testnet

```shell
make init_wallet_testnet
make request_chain_testnet
make build
make publish
# fc7db02564389ad2886598769bfe2675c391970ae9155ca15b28fe1e8c0bdade
# Get your current chain.
linera wallet show
# 787eda3c41d105ee9b3eb271dbafc182c173cbb77f0d70c183ece2ea85570c10
make local_service
# Go to http://localhost:7070/chains/787eda3c41d105ee9b3eb271dbafc182c173cbb77f0d70c183ece2ea85570c10/applications/fc7db02564389ad2886598769bfe2675c391970ae9155ca15b28fe1e8c0bdade
```

### Local

1. Remove already defined local keys and chains.

```shell
rm -rf /Users/lavr/Library/Application\ Support/linera
```

This is a default location on macOS.


2. Run local Linera network.

```shell
make local_net
```

3. Init local wallet.

```shell
make init_wallet_local
```

4. Request local chain.

```shell
make request_chain_local
```

5. Build your source code.

```shell
make build
```

6. Publish your application.

```shell
make publish
```

Somewhere save application id.

7. Start local Linera service.

```shell
make local_service
```

8. Get your default local chain.

```shell
linera wallet show
```

Get first chain id and save it somewhere.

9. You can use application using GraphQL API

Go to `http://localhost:7070/chains/<chain_id>/applications/<application_id>` and execute GraphQL queries.

## GraphQL examples

Land data on-chain

```text
mutation {
  land(
    id: "6a300ad5-15c5-4ac6-be27-b8a4d1d972ee"
    hash: "903b1c65f8ad53b2acf8704cf2ae766eae37eac0b5196321f34c3e07df3ecf30"
    signature: "2c546b2937ea6452c1f381d0c20077f02f63af72f77c1ed76edd4e220e2f59733fada263eab046c55d89a1ca8a6e3504d65eb4d6f40205c3cc7ac2c4603ffb04"
  )
}
```

Query data

```text
{
  value(id: "6a300ad5-15c5-4ac6-be27-b8a4d1d972ee") {
    hash
    signature
  }
}
```
