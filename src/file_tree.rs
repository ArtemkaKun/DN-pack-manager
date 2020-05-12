pub struct tree {
    tree_list: Vec<one_node>,
}

struct one_node {
    name: String,
}

impl tree {
    pub fn init_tree(&mut self) {
        self.tree_list =Vec::<one_node>::new();
    }

    pub fn add_node(&mut self, node_name: String) {
        let new_node = one_node {name: node_name,};
        self.tree_list.push(new_node);
    }
}
