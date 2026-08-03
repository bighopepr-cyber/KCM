# kcm-cluster

Cluster management tool for KCM.

## Status: Planned

## Commands

| Command | Description |
|---------|-------------|
| kcm-cluster status | Show cluster status |
| kcm-cluster nodes | List cluster nodes |
| kcm-cluster add-node <addr> | Add node to cluster |
| kcm-cluster remove-node <id> | Remove node |
| kcm-cluster rebalance | Rebalance shards |

## Usage

```bash
# Show cluster status
kcm-cluster status

# Add a node
kcm-cluster add-node 192.168.1.100:8080
```
