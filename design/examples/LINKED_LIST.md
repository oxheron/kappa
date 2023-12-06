Linked list ideas

```
struct Node {
    Link link; 
    i64 data;
}

struct Link {
    Node* node;
}

func add_node (Link** link, i64 data) -> (Link**) {
    // Goto last node
    // Need a way to store the pointers without owning them 
    // Or some sort of "valid function" that can do this because it's technically well defined
    
    if (link.node) 
    {
        // Implictly this will set link.node.link to the rval
        add_node(link.node.link, data)
    }
    else 
    {
        link.node = new Node;
        link.node.link.node = nullptr;
        link.node.data = data;
    }

    return link
}

func insert_node(Link** link, i64 data, u64 idx, u64 current = 0) -> (Link**) {
    // Prolly bad edge case but this just for decision
    if (!link.node) error!!! 

    if (current == idx) 
    {
        let node = new Node;
        node.data = data;
        node.link = link.node.link;
        link.node = node;
    }
    else 
    {
        insert_node(link.node.link, data, idx, current + 1);
    }

    return link
}

func remove_node(Link** link, i64 data, u64 idx, u64 current = 0) -> (Link**) {
    if (current )
}

```
