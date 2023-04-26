pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Debug, Clone)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from,
                        to,
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).copied()
                }

                pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Edge<'a> {
                    self.attrs = HashMap::from_iter(attrs.iter().map(|(x, y)| (*x, *y)));
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Debug, Clone)]
            pub struct Node<'a> {
                pub name: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Node {
                        name,
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).copied()
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Node<'a> {
                    self.attrs = HashMap::from_iter(attrs.iter().map(|(x, y)| (*x, *y)));
                    self
                }
            }
        }
    }

    pub struct Graph<'a> {
        pub edges: Vec<Edge<'a>>,
        pub nodes: Vec<Node<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                edges: vec![],
                nodes: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|x| x.name == name)
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node<'a>>) -> Graph<'a> {
            self.nodes = nodes.clone();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge<'a>>) -> Graph<'a> {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Graph<'a> {
            self.attrs =
                HashMap::from_iter(attrs.iter().map(|(x, y)| (x.to_string(), y.to_string())));
            self
        }
    }
}
