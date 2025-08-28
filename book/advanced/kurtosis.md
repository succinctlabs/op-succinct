# Using Kurtosis to test `op-succinct`

## What is Kurtosis?

Kurtosis is a development tool that allows you spin up local testnets for blockchain development and testing. The ethPandaOps team has created a package for spinning up OP Stack devnets using Kurtosis called [optimism-package](https://github.com/ethpandaops/optimism-package).

## Install Kurtosis

First, install Kurtosis by following the instructions here: [Kurtosis Installation Guide](https://docs.kurtosis.com/install/).

## How to configure the OP Stack devnet

Configure the `op-network.yaml` file to use the Kurtosis engine:

```yaml
optimism_package:
  chains:
    op-succinct-base:
      participants:
        node0:
          el:
            type: op-geth
          cl:
            type: op-node
      network_params:
        network_id: "2151908"
        seconds_per_slot: 2
        fjord_time_offset: 0
        granite_time_offset: 0
        holocene_time_offset: 0
        fund_dev_accounts: true
  global_log_level: "info"
  global_node_selectors: {}
  global_tolerations: []
  persistent: false
ethereum_package:
  port_publisher:
    nat_exit_ip: KURTOSIS_IP_ADDR_PLACEHOLDER
    el:
      enabled: true
      public_port_start: 52000
    cl:
      enabled: true
      public_port_start: 53000
    vc:
      enabled: true
      public_port_start: 54000
    remote_signer:
      enabled: true
      public_port_start: 55000
    additional_services:
      enabled: true
      public_port_start: 56000
  network_params:
    network_id: "31337"
    seconds_per_slot: 12
    eth1_follow_distance: 2048
    min_validator_withdrawability_delay: 256
    shard_committee_period: 256
    preset: minimal
    genesis_delay: 12
    electra_fork_epoch: 1
    additional_preloaded_contracts: '
      {
        "0x4e59b44847b379578588920cA78FbF26c0B4956C": {
          "balance": "0ETH",
          "code": "0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3",
          "storage": {},
          "nonce": "1"
        }
      }
    '
```

By default it will not start the explorer, we can add this configs for boot them:

```yaml
optimism_package:
  chains:
    op-succinct-base:
      blockscout_params:
        enabled: True
ethereum_package:
  additional_services:
    - dora
    - blockscout
```

## How to run Kurtosis?

Run the testnet using the following command:

```bash
kurtosis run --enclave my-testnet github.com/ethpandaops/optimism-package@1.4.0 --args-file op-network.yaml --image-download always
```

## How to get the relevant RPC's from Kurtosis?

Once the Kurtosis service is running, you can get the relevant RPC endpoints (`L1_RPC`, `L2_RPC`, `L1_BEACON_RPC`, `L2_NODE_RPC`) from the logs:

```bash
========================================== User Services ==========================================
UUID           Name                                             Ports                                                Status
3918f01565d0   cl-1-lighthouse-geth                             http: 4000/tcp -> http://127.0.0.1:53001             RUNNING
                                                                metrics: 5054/tcp -> http://127.0.0.1:53002          
                                                                tcp-discovery: 53000/tcp -> 127.0.0.1:53000          
                                                                udp-discovery: 53000/udp -> 127.0.0.1:53000          
2b23aa0108ae   el-1-geth-lighthouse                             engine-rpc: 8551/tcp -> 127.0.0.1:52001              RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:52004          
                                                                rpc: 8545/tcp -> 127.0.0.1:52002                     
                                                                tcp-discovery: 52000/tcp -> 127.0.0.1:52000          
                                                                udp-discovery: 52000/udp -> 127.0.0.1:52000          
                                                                ws: 8546/tcp -> 127.0.0.1:52003                      
0747e401e36d   grafana                                          http: 3000/tcp -> http://127.0.0.1:32975             RUNNING
6a093c6bb1f6   op-batcher-2151908-op-succinct-base              http: 8548/tcp -> http://127.0.0.1:32968             RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:32969          
45a8f7bf3bcb   op-cl-2151908-node0-op-node                      metrics: 9001/tcp -> http://127.0.0.1:32966          RUNNING
                                                                rpc: 8547/tcp -> http://127.0.0.1:32965              
                                                                tcp-discovery: 9003/tcp -> http://127.0.0.1:32967    
                                                                udp-discovery: 9003/udp -> http://127.0.0.1:32802    
b0f4baf05d98   op-el-2151908-node0-op-geth                      engine-rpc: 8551/tcp -> http://127.0.0.1:32962       RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:32963          
                                                                rpc: 8545/tcp -> http://127.0.0.1:32960              
                                                                tcp-discovery: 30303/tcp -> http://127.0.0.1:32964   
                                                                udp-discovery: 30303/udp -> http://127.0.0.1:32801   
                                                                ws: 8546/tcp -> http://127.0.0.1:32961               
014f3a266f62   op-proposer-2151908-op-succinct-base             http: 8560/tcp -> http://127.0.0.1:32970             RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:32971          
0ab72a1fd691   prometheus                                       http: 9090/tcp -> http://127.0.0.1:32974             RUNNING
5d964e9e2ae3   proxyd-2151908-op-succinct-base                  http: 8080/tcp -> http://127.0.0.1:32973             RUNNING
                                                                metrics: 7300/tcp -> http://127.0.0.1:32972          
17e0a590846a   validator-key-generation-cl-validator-keystore   <none>                                               RUNNING
d1da5f20a8ac   vc-1-geth-lighthouse                             metrics: 8080/tcp -> http://127.0.0.1:54000          RUNNING
```

Relevant endpoints:

| Endpoint | Service | URL |
|----------|---------|-----|
| L1_RPC | `rpc` port of `el-1-geth-lighthouse` | `http://127.0.0.1:52002` |
| L2_RPC | `rpc` port of `op-el-2151908-node0-op-geth` | `http://127.0.0.1:32960` |
| L1_BEACON_RPC | `http` port of `cl-1-lighthouse-geth` | `http://127.0.0.1:53001` |
| L2_NODE_RPC | `http` port of `op-cl-2151908-node0-op-node ` | `http://127.0.0.1:32965` |

we can stop the op-proposer by:

```bash
kurtosis service stop my-testnet op-proposer-2151908-op-succinct-base
```

Then we can search the status by:

```bash
kurtosis enclave inspect my-testnet
```

It will show:

```bash
========================================== User Services ==========================================
UUID           Name                                             Ports                                                Status
3918f01565d0   cl-1-lighthouse-geth                             http: 4000/tcp -> http://127.0.0.1:53001             RUNNING
                                                                metrics: 5054/tcp -> http://127.0.0.1:53002          
                                                                tcp-discovery: 53000/tcp -> 127.0.0.1:53000          
                                                                udp-discovery: 53000/udp -> 127.0.0.1:53000          
2b23aa0108ae   el-1-geth-lighthouse                             engine-rpc: 8551/tcp -> 127.0.0.1:52001              RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:52004          
                                                                rpc: 8545/tcp -> 127.0.0.1:52002                     
                                                                tcp-discovery: 52000/tcp -> 127.0.0.1:52000          
                                                                udp-discovery: 52000/udp -> 127.0.0.1:52000          
                                                                ws: 8546/tcp -> 127.0.0.1:52003                      
0747e401e36d   grafana                                          http: 3000/tcp -> http://127.0.0.1:32975             RUNNING
6a093c6bb1f6   op-batcher-2151908-op-succinct-base              http: 8548/tcp -> http://127.0.0.1:32968             RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:32969          
45a8f7bf3bcb   op-cl-2151908-node0-op-node                      metrics: 9001/tcp -> http://127.0.0.1:32966          RUNNING
                                                                rpc: 8547/tcp -> http://127.0.0.1:32965              
                                                                tcp-discovery: 9003/tcp -> http://127.0.0.1:32967    
                                                                udp-discovery: 9003/udp -> http://127.0.0.1:32802    
b0f4baf05d98   op-el-2151908-node0-op-geth                      engine-rpc: 8551/tcp -> http://127.0.0.1:32962       RUNNING
                                                                metrics: 9001/tcp -> http://127.0.0.1:32963          
                                                                rpc: 8545/tcp -> http://127.0.0.1:32960              
                                                                tcp-discovery: 30303/tcp -> http://127.0.0.1:32964   
                                                                udp-discovery: 30303/udp -> http://127.0.0.1:32801   
                                                                ws: 8546/tcp -> http://127.0.0.1:32961               
014f3a266f62   op-proposer-2151908-op-succinct-base             http: 8560/tcp                                       STOPPED
                                                                metrics: 9001/tcp                                    
0ab72a1fd691   prometheus                                       http: 9090/tcp -> http://127.0.0.1:32974             RUNNING
5d964e9e2ae3   proxyd-2151908-op-succinct-base                  http: 8080/tcp -> http://127.0.0.1:32973             RUNNING
                                                                metrics: 7300/tcp -> http://127.0.0.1:32972          
17e0a590846a   validator-key-generation-cl-validator-keystore   <none>                                               RUNNING
d1da5f20a8ac   vc-1-geth-lighthouse                             metrics: 8080/tcp -> http://127.0.0.1:54000          RUNNING
```

Note the  `op-proposer-2151908-op-succinct-base` had stopped.

## Spin down the devnet

Remove the devnet with:

```bash
kurtosis clean -a
```

Or just stop this devnet by name `my-testnet`

```bash
kurtosis enclave rm my-testnet -f 
```