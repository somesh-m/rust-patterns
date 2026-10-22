/*This is a lightweight design pattern with zero cost abstraction, meaning the wrapper is strictly a compile time construct incurring zero performance penalty. If we have multiple parameters that use the same primitive data type it is dangerously easy to mix them up. The newtype patterns forces the compiler to catch these logical errors. This is different from using type, because type creates an alias, compiler internally still sees them as their root primitive type.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ShardId(u16);

fn route_request(node_id: NodeId, shard_id: ShardId) {
    println!("Node: {:?}, Shard: {:?}", node_id.0, shard_id.0);
}

//Additional benefit of using this pattern is that we can also enforce invariants
impl ShardId {
    pub const MAX_SHARDS: u16 = 32;

    pub fn new(value: u16) -> Option<Self> {
        if value < Self::MAX_SHARDS {
            Some(Self(value))
        } else {
            println!("Invalid shard range");
            None
        }
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

fn main() {
    let node_id = NodeId(2);
    let shard_id = ShardId(3);
    let invalid_shard = ShardId::new(59);

    route_request(node_id, shard_id);

    //The below line won't compile even though the base type of both is u64
    // route_request(shard_id, node_id);
}
