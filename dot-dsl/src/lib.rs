pub mod graph {
    use std::collections::HashMap;

    macro_rules! impl_attrs {
        () => {
            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                for (k, v) in attrs.iter() {
                    self.attrs.insert(k.to_string(), v.to_string());
                }
                self
            }
        };
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        impl_attrs!();

        pub fn get_node(&self, node: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|x| x.name == node)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Edge {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                impl_attrs!();
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                impl_attrs!();

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|x| x.as_str())
                }
            }
        }
    }
}
