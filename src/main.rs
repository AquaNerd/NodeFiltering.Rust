#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Node>
}

impl Node {
    // Function to sum the values of all children
    fn sum_children(&self) -> i32 {
        self.children.iter().map(|child| child.value).sum()
    }
}

fn main() {
    let nodes = vec![
        Node { value: 1, 
            children: vec![
                Node { value: 0, children: vec![] }, 
                Node { value: 0, children: vec![] }] },
        Node { value: 2, 
            children: vec![
                Node { value: 1, children: vec![] }, 
                Node { value: -1, children: vec![] }] },
        Node { value: 3, 
            children: vec![
                Node { value: 1, children: vec![] }, 
                Node { value: 1, children: vec![] }] },
    ];

    // Filter out nodes whose children sum to zero
    let filtered_nodes: Vec<Node> = nodes.into_iter()
        .filter_map(|node| {
            if node.sum_children() != 0 {
                Some(node)
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", filtered_nodes);
}
