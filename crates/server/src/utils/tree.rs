use serde::Serialize;
use std::collections::HashMap;

/// 树的节点
#[derive(Serialize)]
pub struct Node<T> {
    item: T,
    children: Option<Vec<Node<T>>>,
}
pub trait TreeNode {
    type Id: std::cmp::Eq + std::hash::Hash;
    fn key(&self) -> Self::Id;
    fn parent_key(&self) -> Option<Self::Id>;
}

#[derive(Serialize)]
pub struct Tree<T> {
    nodes: Option<Vec<Node<T>>>,
}
impl<T> From<Vec<T>> for Tree<T>
where
    T: TreeNode,
{
    fn from(value: Vec<T>) -> Self {
        let mut map = std::collections::HashMap::new();
        for item in value {
            map.entry(item.parent_key())
                .or_insert_with(Vec::new)
                .push(Node {
                    item,
                    children: None,
                });
        }
        let nodes = build(&mut map, None);
        Tree { nodes }
    }
}
fn build<T>(
    map: &mut HashMap<Option<T::Id>, Vec<Node<T>>>,
    root: Option<T::Id>,
) -> Option<Vec<Node<T>>>
where
    T: TreeNode,
{
    map.remove(&root).map(|mut children| {
        for child in &mut children {
            child.children = build(map, Some(child.item.key()))
        }
        children
    })
}
