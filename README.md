# raft

### **引言**

之前在准备面试时，看了陈皓老师关于分布式锁的分享（[YouTube 链接](https://www.youtube.com/watch?v=vFW1U1vimVs)），还有一些关于分布式事务的资料，以及一部分《数据密集型应用系统设计》，在接触了这些之后，想更深入了解下用于实现多数据节点间状态同步的分布式共识协议，这个月详细看了下 Raft 协议的 paper（[PDF 链接](https://raft.github.io/raft.pdf)），打算写这篇文章来记录下自己的思考，也想自己实现下这个协议。

### **初探**

最早的分布式共识协议是 [Paxos](https://zh.wikipedia.org/wiki/Paxos%E7%AE%97%E6%B3%95)，但 Paxos 难于理解且很难直接应用于实际环境，在这个背景下，斯坦福大学团队希望能构建一个更易理解及能够直接应用于实际环境的分布式共识协议，最终提出了 Raft 协议。

为了易于理解，Raft 协议将共识协议分为几个独立模块：

1. Leader 选举
2. 日志复制
3. 安全性

- 需要搭建一个网络框架，用于数据节点间的互相通信
- 需要支持配置文件
- 每个请求需要有唯一的序列号，用于保证请求的幂等性
- 只有 leader 节点可以处理写请求
- 为保证返回最新的数据，只有 leader 节点可以处理读请求
- 需要实现一个定时任务框架，用于实现 leader 节点与数据节点间定时的心跳检测

从一个新创建的数据节点出发。

当一个节点想要加入一个集群时，**首先需要找到该集群的 leader 节点**，该节点会向集群中的任意节点发起请求，如果接收到请求的节点不是 leader 节点，那么此节点会拒绝这个请求，并返回其最近知晓的 leader 节点的信息（参考 paper 第 8 节）

**第二步数据节点需要追赶 leader 节点的数据状态。**这个过程是通过 AppendEntries RPC 完成的，AppendEntries RPC 具有一致性检查功能。

leader 节点需要维护每个数据节点的`nextIndex`。

log entry 包含 term 与 log index。

> Once a follower learns that a log entry is committed, it applies the entry to its local state machine (in log order).

> Log Matching: if two logs contain an entry with the same index and term, then the logs are identical in all entries up through the given index. §5.3

> In Raft, the leader handles inconsistencies by forcing the followers' logs to duplicate its own.

``` json 
{
    "server_id": 1,
    "listen_address": "127.0.0.1:3456",
    "nodes": [
        "127.0.0.1:8081",
        "127.0.0.1:8082"
    ]
}
```

