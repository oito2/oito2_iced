// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! An immutable tree of nodes with the queries a tree view needs.

use std::collections::{HashMap, HashSet};

use super::CollectionError;
use super::text_folding::fold_text;

/// A node of a [`TreeCollection`]: a unique `value`, a label, an optional
/// payload and, for branches, its children.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    /// The unique value of the node in its tree.
    pub value: String,
    /// The display label; the value is shown when it is `None`.
    pub label: Option<String>,
    /// The payload, such as the file or record the node shows.
    pub data: Option<T>,
    /// The child nodes of a branch; `None` for a leaf.
    pub children: Option<Vec<TreeNode<T>>>,
    /// Whether the node can't be selected or focused.
    pub disabled: bool,
}

impl<T> TreeNode<T> {
    /// Creates a leaf with `value`; the label defaults to the value.
    #[must_use]
    pub fn leaf(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
            data: None,
            children: None,
            disabled: false,
        }
    }

    /// Creates a branch with `value` and `children` (possibly empty).
    #[must_use]
    pub fn branch(value: impl Into<String>, children: Vec<TreeNode<T>>) -> Self {
        Self {
            children: Some(children),
            ..Self::leaf(value)
        }
    }

    /// Returns the node with the display `label`.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the node with the payload `data`.
    #[must_use]
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Returns the node with its disabled state set to `disabled`.
    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The display label; defaults to the value.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.value)
    }

    /// Whether the node can have children.
    #[must_use]
    pub fn is_branch(&self) -> bool {
        self.children.is_some()
    }

    /// The children of a branch; empty for a leaf.
    fn kids(&self) -> &[TreeNode<T>] {
        self.children.as_deref().unwrap_or(&[])
    }
}

/// A fallible rewrite of one node.
type NodeEdit<'a, T> = &'a dyn Fn(&TreeNode<T>) -> Result<TreeNode<T>, CollectionError>;

/// A node with its position in a flattened [`TreeCollection`].
#[derive(Debug, Clone, PartialEq)]
pub struct TreeEntry<'a, T> {
    /// The node.
    pub node: &'a TreeNode<T>,
    /// The number of ancestors of the node.
    pub depth: usize,
    /// The child indexes from the top level down to the node.
    pub index_path: Vec<usize>,
}

/// An immutable tree of [`TreeNode`]s with the queries a tree view needs:
/// lookups by value and index path, parents and siblings, the visible order
/// for a set of expanded branches, filtering and copies with nodes inserted,
/// removed, moved or replaced.
///
/// An index path lists the child indexes from the top level down, such as
/// `[0, 2]` for the third child of the first top-level node.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeCollection<T> {
    roots: Vec<TreeNode<T>>,
    paths: HashMap<String, Vec<usize>>,
    parents: HashMap<String, Option<String>>,
}

impl<T> Default for TreeCollection<T> {
    fn default() -> Self {
        Self {
            roots: Vec::new(),
            paths: HashMap::new(),
            parents: HashMap::new(),
        }
    }
}

impl<T> TreeCollection<T> {
    /// Creates a tree with the top-level `roots`.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::DuplicateValue`] when two nodes share a
    /// value.
    pub fn new(roots: Vec<TreeNode<T>>) -> Result<Self, CollectionError> {
        fn index<T>(
            nodes: &[TreeNode<T>],
            path: &[usize],
            parent: Option<&str>,
            paths: &mut HashMap<String, Vec<usize>>,
            parents: &mut HashMap<String, Option<String>>,
        ) -> Result<(), CollectionError> {
            for (i, node) in nodes.iter().enumerate() {
                let mut node_path = path.to_vec();
                node_path.push(i);
                if paths.contains_key(&node.value) {
                    return Err(CollectionError::DuplicateValue(node.value.clone()));
                }
                paths.insert(node.value.clone(), node_path.clone());
                parents.insert(node.value.clone(), parent.map(str::to_owned));
                index(node.kids(), &node_path, Some(&node.value), paths, parents)?;
            }
            Ok(())
        }

        let mut paths = HashMap::new();
        let mut parents = HashMap::new();
        index(&roots, &[], None, &mut paths, &mut parents)?;
        Ok(Self {
            roots,
            paths,
            parents,
        })
    }

    /// Builds a tree from `paths` split by `separator`, such as file paths:
    /// each segment becomes a branch and the last one a leaf, unless another
    /// path continues below it. Empty segments are skipped. Node values are
    /// the paths up to each node, joined by `separator`, and labels are the
    /// segment names.
    #[must_use]
    pub fn from_paths<S: AsRef<str>>(paths: impl IntoIterator<Item = S>, separator: &str) -> Self {
        let mut children: HashMap<String, Vec<String>> = HashMap::new();
        children.insert(String::new(), Vec::new());
        let mut leaves: HashSet<String> = HashSet::new();
        for path in paths {
            let parts: Vec<&str> = path
                .as_ref()
                .split(separator)
                .filter(|part| !part.is_empty())
                .collect();
            let mut parent = String::new();
            for (i, part) in parts.iter().enumerate() {
                let value = if parent.is_empty() {
                    (*part).to_owned()
                } else {
                    format!("{parent}{separator}{part}")
                };
                let siblings = children.entry(parent.clone()).or_default();
                if !siblings.contains(&value) {
                    siblings.push(value.clone());
                }
                if i == parts.len() - 1 {
                    leaves.insert(value.clone());
                }
                children.entry(value.clone()).or_default();
                parent = value;
            }
        }

        fn build<T>(
            value: &str,
            separator: &str,
            children: &HashMap<String, Vec<String>>,
            leaves: &HashSet<String>,
        ) -> TreeNode<T> {
            let kids = children.get(value).map_or(&[][..], Vec::as_slice);
            let label = value.rsplit(separator).next().unwrap_or(value).to_owned();
            let mut node = TreeNode::leaf(value).with_label(label);
            if !(kids.is_empty() && leaves.contains(value)) {
                node.children = Some(
                    kids.iter()
                        .map(|kid| build(kid, separator, children, leaves))
                        .collect(),
                );
            }
            node
        }

        let roots = children
            .get("")
            .map_or(&[][..], Vec::as_slice)
            .iter()
            .map(|root| build(root, separator, &children, &leaves))
            .collect();
        // Values are unique by construction; an empty tree is the fallback.
        Self::new(roots).unwrap_or_default()
    }

    /// The top-level nodes.
    #[must_use]
    pub fn roots(&self) -> &[TreeNode<T>] {
        &self.roots
    }

    /// Whether a node has `value`.
    #[must_use]
    pub fn has(&self, value: &str) -> bool {
        self.paths.contains_key(value)
    }

    /// Returns the index path of the node with `value`, or `None` when there
    /// is none.
    #[must_use]
    pub fn index_path_of(&self, value: &str) -> Option<&[usize]> {
        self.paths.get(value).map(Vec::as_slice)
    }

    /// Returns the node at `index_path`, or `None` when the path is empty or
    /// doesn't exist.
    #[must_use]
    pub fn node_at(&self, index_path: &[usize]) -> Option<&TreeNode<T>> {
        let mut level: &[TreeNode<T>] = &self.roots;
        let mut node = None;
        for &index in index_path {
            let current = level.get(index)?;
            level = current.kids();
            node = Some(current);
        }
        node
    }

    /// Returns the node with `value`, or `None` when there is none.
    #[must_use]
    pub fn find(&self, value: &str) -> Option<&TreeNode<T>> {
        self.node_at(self.paths.get(value)?)
    }

    /// Returns the value of the parent of `value`, or `None` for a top-level
    /// or unknown node.
    #[must_use]
    pub fn parent_of(&self, value: &str) -> Option<&str> {
        self.parents.get(value)?.as_deref()
    }

    /// Returns the children of `value`; empty for a leaf or a missing node.
    #[must_use]
    pub fn children_of(&self, value: &str) -> &[TreeNode<T>] {
        self.find(value).map_or(&[], TreeNode::kids)
    }

    /// Returns the nodes that share a parent with `value`, including it; the
    /// top-level nodes for an unknown value.
    #[must_use]
    pub fn siblings_of(&self, value: &str) -> &[TreeNode<T>] {
        match self.parent_of(value) {
            Some(parent) => self.children_of(parent),
            None => &self.roots,
        }
    }

    /// Returns the values of the ancestors of `value`, from the top level
    /// down.
    #[must_use]
    pub fn ancestors_of(&self, value: &str) -> Vec<&str> {
        let mut result = Vec::new();
        let mut current = self.parent_of(value);
        while let Some(parent) = current {
            result.push(parent);
            current = self.parent_of(parent);
        }
        result.reverse();
        result
    }

    /// Returns the depth of `value`: 0 at the top level, and also 0 for an
    /// unknown value.
    #[must_use]
    pub fn depth_of(&self, value: &str) -> usize {
        self.paths.get(value).map_or(0, |path| path.len() - 1)
    }

    /// Returns every node in depth-first order with its depth and index
    /// path.
    #[must_use]
    pub fn flatten(&self) -> Vec<TreeEntry<'_, T>> {
        self.walk(None)
    }

    /// Returns the nodes a tree view shows when the branches in `expanded`
    /// are open, in display order.
    #[must_use]
    pub fn visible(&self, expanded: &HashSet<String>) -> Vec<TreeEntry<'_, T>> {
        self.walk(Some(expanded))
    }

    /// Returns the nodes in depth-first order, descending only into the
    /// branches in `expanded`, or into every branch when it is `None`.
    fn walk(&self, expanded: Option<&HashSet<String>>) -> Vec<TreeEntry<'_, T>> {
        fn go<'a, T>(
            nodes: &'a [TreeNode<T>],
            path: &[usize],
            expanded: Option<&HashSet<String>>,
            result: &mut Vec<TreeEntry<'a, T>>,
        ) {
            for (i, node) in nodes.iter().enumerate() {
                let mut node_path = path.to_vec();
                node_path.push(i);
                result.push(TreeEntry {
                    node,
                    depth: path.len(),
                    index_path: node_path.clone(),
                });
                if expanded.is_none_or(|set| set.contains(&node.value)) {
                    go(node.kids(), &node_path, expanded, result);
                }
            }
        }

        let mut result = Vec::new();
        go(&self.roots, &[], expanded, &mut result);
        result
    }

    /// The values of every node, in depth-first order.
    #[must_use]
    pub fn values(&self) -> Vec<&str> {
        self.flatten()
            .into_iter()
            .map(|entry| entry.node.value.as_str())
            .collect()
    }

    /// The values of every branch, in depth-first order.
    #[must_use]
    pub fn branch_values(&self) -> Vec<&str> {
        self.flatten()
            .into_iter()
            .filter(|entry| entry.node.is_branch())
            .map(|entry| entry.node.value.as_str())
            .collect()
    }

    /// Returns the values of every node below `value`, in depth-first order.
    #[must_use]
    pub fn descendants_of(&self, value: &str) -> Vec<&str> {
        fn go<'a, T>(nodes: &'a [TreeNode<T>], result: &mut Vec<&'a str>) {
            for node in nodes {
                result.push(&node.value);
                go(node.kids(), result);
            }
        }

        let mut result = Vec::new();
        go(self.children_of(value), &mut result);
        result
    }

    /// Returns the values of the leaves below `value`, or `value` itself
    /// when it is a leaf; empty for an unknown value. Empty branches don't
    /// count as leaves.
    #[must_use]
    pub fn leaves_of(&self, value: &str) -> Vec<&str> {
        let Some(node) = self.find(value) else {
            return Vec::new();
        };
        if !node.is_branch() {
            return vec![node.value.as_str()];
        }
        self.descendants_of(value)
            .into_iter()
            .filter(|descendant| !self.find(descendant).is_some_and(TreeNode::is_branch))
            .collect()
    }

    /// Returns the value of the enabled visible node after `value` for the
    /// `expanded` branches, or `None` at the end or when `value` isn't
    /// visible.
    #[must_use]
    pub fn next_visible(&self, value: &str, expanded: &HashSet<String>) -> Option<&str> {
        self.step(value, expanded, 1)
    }

    /// Returns the value of the enabled visible node before `value` for the
    /// `expanded` branches, or `None` at the start or when `value` isn't
    /// visible.
    #[must_use]
    pub fn previous_visible(&self, value: &str, expanded: &HashSet<String>) -> Option<&str> {
        self.step(value, expanded, -1)
    }

    /// Returns the value of the first enabled visible node found by moving
    /// `step` positions at a time from `value`.
    fn step(&self, value: &str, expanded: &HashSet<String>, step: isize) -> Option<&str> {
        let list = self.visible(expanded);
        let mut index = list
            .iter()
            .position(|entry| entry.node.value == value)?
            .cast_signed();
        let length = list.len().cast_signed();
        index += step;
        while index >= 0 && index < length {
            let entry = &list[index.cast_unsigned()];
            if !entry.node.disabled {
                return Some(entry.node.value.as_str());
            }
            index += step;
        }
        None
    }
}

impl<T> TreeCollection<T>
where
    T: Clone,
{
    /// Returns a tree with the nodes for which `test` returns true and their
    /// ancestors. A matching node keeps all its descendants; a branch that
    /// doesn't match is kept, with only its matching descendants, when any
    /// of them matches.
    #[must_use]
    pub fn filter(&self, test: impl Fn(&TreeNode<T>) -> bool) -> Self {
        fn keep<T: Clone>(
            nodes: &[TreeNode<T>],
            test: &dyn Fn(&TreeNode<T>) -> bool,
        ) -> Vec<TreeNode<T>> {
            let mut result = Vec::new();
            for node in nodes {
                if test(node) {
                    result.push(node.clone());
                } else {
                    let kids = keep(node.kids(), test);
                    if !kids.is_empty() {
                        let mut copy = node.clone();
                        copy.children = Some(kids);
                        result.push(copy);
                    }
                }
            }
            result
        }

        // The kept nodes are a subset of a tree with unique values.
        Self::new(keep(&self.roots, &test)).unwrap_or_default()
    }

    /// Returns a tree with the nodes whose label contains `query`, ignoring
    /// case and accents, and their ancestors, as [`filter`](Self::filter)
    /// keeps them.
    #[must_use]
    pub fn search(&self, query: &str) -> Self {
        let needle = fold_text(query);
        self.filter(|node| fold_text(node.label()).contains(&needle))
    }

    /// Returns a tree with `nodes` inserted among the children of
    /// `parent_value` (the top level when `None`) at `index`, or at the end
    /// when `index` is `None`. Inserting into a leaf turns it into a branch.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::UnknownNode`] when `parent_value` isn't in
    /// the tree, [`CollectionError::DuplicateValue`] when a value of `nodes`
    /// is already used, and [`CollectionError::IndexOutOfRange`] when
    /// `index` is greater than the number of children.
    pub fn insert(
        &self,
        parent_value: Option<&str>,
        nodes: Vec<TreeNode<T>>,
        index: Option<usize>,
    ) -> Result<Self, CollectionError> {
        if let Some(parent) = parent_value
            && !self.has(parent)
        {
            return Err(CollectionError::UnknownNode(parent.to_owned()));
        }
        let into = |list: &[TreeNode<T>]| -> Result<Vec<TreeNode<T>>, CollectionError> {
            let at = index.unwrap_or(list.len());
            if at > list.len() {
                return Err(CollectionError::IndexOutOfRange {
                    index: at,
                    len: list.len() + 1,
                });
            }
            let mut copy = list.to_vec();
            copy.splice(at..at, nodes.iter().cloned());
            Ok(copy)
        };
        match parent_value {
            None => Self::new(into(&self.roots)?),
            Some(parent) => self.map_node(parent, |node| {
                let mut copy = node.clone();
                copy.children = Some(into(node.kids())?);
                Ok(copy)
            }),
        }
    }

    /// Returns a tree without the nodes in `values` and their descendants;
    /// unknown values are ignored.
    #[must_use]
    pub fn remove<S: AsRef<str>>(&self, values: impl IntoIterator<Item = S>) -> Self {
        let set: HashSet<String> = values
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .collect();
        self.without(&set)
    }

    fn without(&self, set: &HashSet<String>) -> Self {
        fn prune<T: Clone>(nodes: &[TreeNode<T>], set: &HashSet<String>) -> Vec<TreeNode<T>> {
            nodes
                .iter()
                .filter(|node| !set.contains(&node.value))
                .map(|node| {
                    let mut copy = node.clone();
                    if let Some(children) = &node.children {
                        copy.children = Some(prune(children, set));
                    }
                    copy
                })
                .collect()
        }

        // A subset of a tree with unique values keeps them unique.
        Self::new(prune(&self.roots, set)).unwrap_or_default()
    }

    /// Returns a tree with the nodes in `values` moved, in tree order, among
    /// the children of `parent_value` (the top level when `None`) at
    /// `index`, or at the end when `index` is `None`. `index` counts the
    /// children left after the moved nodes are taken out. A moved node
    /// brings its descendants along, and unknown values are ignored.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::UnknownNode`] when `parent_value` isn't in
    /// the tree, [`CollectionError::InvalidMove`] when it is one of the
    /// moved nodes or a descendant of one, and
    /// [`CollectionError::IndexOutOfRange`] when `index` is greater than the
    /// number of remaining children.
    pub fn move_nodes<S: AsRef<str>>(
        &self,
        values: impl IntoIterator<Item = S>,
        parent_value: Option<&str>,
        index: Option<usize>,
    ) -> Result<Self, CollectionError> {
        let set: HashSet<String> = values
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .collect();
        if let Some(parent) = parent_value {
            if !self.has(parent) {
                return Err(CollectionError::UnknownNode(parent.to_owned()));
            }
            if set.contains(parent)
                || self
                    .ancestors_of(parent)
                    .iter()
                    .any(|ancestor| set.contains(*ancestor))
            {
                return Err(CollectionError::InvalidMove(parent.to_owned()));
            }
        }
        let moved: Vec<TreeNode<T>> = self
            .flatten()
            .into_iter()
            .filter(|entry| {
                set.contains(&entry.node.value)
                    && !self
                        .ancestors_of(&entry.node.value)
                        .iter()
                        .any(|ancestor| set.contains(*ancestor))
            })
            .map(|entry| entry.node.clone())
            .collect();
        self.without(&set).insert(parent_value, moved, index)
    }

    /// Returns a tree with the node `value` replaced by `replacement`, or an
    /// equal tree when `value` is unknown.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::DuplicateValue`] when `replacement` brings
    /// a value already used elsewhere in the tree.
    pub fn replace(&self, value: &str, replacement: TreeNode<T>) -> Result<Self, CollectionError> {
        self.map_node(value, |_| Ok(replacement.clone()))
    }

    /// Returns a tree with the node `value` replaced by the result of
    /// `change`.
    fn map_node(
        &self,
        value: &str,
        change: impl Fn(&TreeNode<T>) -> Result<TreeNode<T>, CollectionError>,
    ) -> Result<Self, CollectionError> {
        fn map<T: Clone>(
            nodes: &[TreeNode<T>],
            value: &str,
            change: NodeEdit<'_, T>,
        ) -> Result<Vec<TreeNode<T>>, CollectionError> {
            nodes
                .iter()
                .map(|node| {
                    if node.value == value {
                        change(node)
                    } else if let Some(children) = &node.children {
                        let mut copy = node.clone();
                        copy.children = Some(map(children, value, change)?);
                        Ok(copy)
                    } else {
                        Ok(node.clone())
                    }
                })
                .collect()
        }

        Self::new(map(&self.roots, value, &change)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TreeCollection<()> {
        TreeCollection::from_paths(
            [
                "src/app.dart",
                "src/widgets/button.dart",
                "src/widgets/field.dart",
                "test/app_test.dart",
                "README.md",
            ],
            "/",
        )
    }

    fn values(nodes: &[TreeNode<()>]) -> Vec<&str> {
        nodes.iter().map(|n| n.value.as_str()).collect()
    }

    fn set(values: &[&str]) -> HashSet<String> {
        values.iter().map(|v| (*v).to_owned()).collect()
    }

    #[test]
    fn builds_branches_and_leaves_from_paths() {
        let tree = sample();
        assert_eq!(values(tree.roots()), ["src", "test", "README.md"]);
        assert!(tree.find("src/widgets").is_some_and(TreeNode::is_branch));
        assert!(!tree.find("src/app.dart").is_some_and(TreeNode::is_branch));
        assert_eq!(
            tree.find("src/widgets/field.dart").map(TreeNode::label),
            Some("field.dart")
        );
    }

    #[test]
    fn from_paths_skips_empty_segments_and_a_path_continuing_below_a_leaf() {
        let tree: TreeCollection<()> = TreeCollection::from_paths(["/a//b", "a", "a/b"], "/");
        assert_eq!(tree.values(), ["a", "a/b"]);
        assert!(tree.find("a").is_some_and(TreeNode::is_branch));
        assert!(!tree.find("a/b").is_some_and(TreeNode::is_branch));
        let other: TreeCollection<()> = TreeCollection::from_paths(["x\\y"], "\\");
        assert_eq!(other.find("x\\y").map(TreeNode::label), Some("y"));
    }

    #[test]
    fn rejects_duplicate_values_and_lists_leaves() {
        let duplicates = TreeCollection::<()>::new(vec![TreeNode::leaf("a"), TreeNode::leaf("a")]);
        assert_eq!(
            duplicates.unwrap_err(),
            CollectionError::DuplicateValue("a".to_owned())
        );
        let nested = TreeCollection::<()>::new(vec![
            TreeNode::branch("a", vec![TreeNode::leaf("b")]),
            TreeNode::leaf("b"),
        ]);
        assert!(nested.is_err());
        let tree = sample();
        assert_eq!(
            tree.leaves_of("src"),
            [
                "src/app.dart",
                "src/widgets/button.dart",
                "src/widgets/field.dart"
            ]
        );
        assert_eq!(tree.leaves_of("README.md"), ["README.md"]);
        assert!(tree.leaves_of("missing").is_empty());
    }

    #[test]
    fn empty_branches_are_not_leaves() {
        let tree = TreeCollection::<()>::new(vec![TreeNode::branch(
            "a",
            vec![TreeNode::branch("b", vec![]), TreeNode::leaf("c")],
        )])
        .expect("unique");
        assert_eq!(tree.leaves_of("a"), ["c"]);
        assert_eq!(tree.branch_values(), ["a", "b"]);
    }

    #[test]
    fn answers_parents_siblings_ancestors_and_depth() {
        let tree = sample();
        assert_eq!(
            tree.parent_of("src/widgets/button.dart"),
            Some("src/widgets")
        );
        assert_eq!(tree.parent_of("src"), None);
        assert_eq!(
            tree.ancestors_of("src/widgets/field.dart"),
            ["src", "src/widgets"]
        );
        assert_eq!(tree.depth_of("src/widgets/field.dart"), 2);
        assert_eq!(tree.depth_of("missing"), 0);
        assert_eq!(
            tree.index_path_of("src/widgets/field.dart"),
            Some(&[0, 1, 1][..])
        );
        assert_eq!(
            tree.node_at(&[0, 1, 1]).map(|n| n.value.as_str()),
            Some("src/widgets/field.dart")
        );
        assert!(tree.node_at(&[]).is_none());
        assert!(tree.node_at(&[9]).is_none());
        assert!(tree.node_at(&[1, 0, 0]).is_none());
        assert_eq!(
            values(tree.siblings_of("test")),
            ["src", "test", "README.md"]
        );
        assert_eq!(
            tree.descendants_of("src"),
            [
                "src/app.dart",
                "src/widgets",
                "src/widgets/button.dart",
                "src/widgets/field.dart"
            ]
        );
        assert!(tree.children_of("README.md").is_empty());
    }

    #[test]
    fn flatten_reports_depth_and_index_paths() {
        let tree = sample();
        let entries = tree.flatten();
        assert_eq!(entries.len(), 8);
        assert_eq!(entries[3].node.value, "src/widgets/button.dart");
        assert_eq!(entries[3].depth, 2);
        assert_eq!(entries[3].index_path, [0, 1, 0]);
    }

    #[test]
    fn lists_and_walks_the_visible_nodes_of_expanded_branches() {
        let tree = sample();
        let expanded = set(&["src"]);
        let visible: Vec<&str> = tree
            .visible(&expanded)
            .iter()
            .map(|e| e.node.value.as_str())
            .collect();
        assert_eq!(
            visible,
            ["src", "src/app.dart", "src/widgets", "test", "README.md"]
        );
        assert_eq!(tree.next_visible("src/widgets", &expanded), Some("test"));
        assert_eq!(
            tree.previous_visible("test", &expanded),
            Some("src/widgets")
        );
        assert_eq!(tree.next_visible("README.md", &expanded), None);
        assert_eq!(
            tree.next_visible("src/widgets/button.dart", &expanded),
            None
        );
    }

    #[test]
    fn visible_navigation_skips_disabled_nodes() {
        let tree = TreeCollection::<()>::new(vec![
            TreeNode::leaf("a"),
            TreeNode::leaf("b").with_disabled(true),
            TreeNode::leaf("c"),
        ])
        .expect("unique");
        let none = HashSet::new();
        assert_eq!(tree.next_visible("a", &none), Some("c"));
        assert_eq!(tree.previous_visible("c", &none), Some("a"));
    }

    #[test]
    fn filters_keep_the_ancestors_of_matches() {
        let tree = sample();
        let found = tree.search("FIELD");
        assert_eq!(
            found.values(),
            ["src", "src/widgets", "src/widgets/field.dart"]
        );
        let whole = tree.filter(|n| n.value == "src");
        assert_eq!(whole.values().len(), 5);
    }

    #[test]
    fn inserts_into_new_trees() {
        let tree = sample();
        let inserted = tree
            .insert(
                Some("test"),
                vec![TreeNode::leaf("test/widget_test.dart")],
                Some(0),
            )
            .expect("inserts");
        assert_eq!(
            values(inserted.children_of("test")),
            ["test/widget_test.dart", "test/app_test.dart"]
        );
        let at_end = tree
            .insert(None, vec![TreeNode::leaf("LICENSE")], None)
            .expect("inserts");
        assert_eq!(
            values(at_end.roots()),
            ["src", "test", "README.md", "LICENSE"]
        );
        let into_leaf = tree
            .insert(Some("README.md"), vec![TreeNode::leaf("x")], None)
            .expect("inserts");
        assert!(into_leaf.find("README.md").is_some_and(TreeNode::is_branch));
        assert!(tree.find("README.md").is_some_and(|n| !n.is_branch()));
    }

    #[test]
    fn insert_rejects_unknown_parents_duplicates_and_bad_indexes() {
        let tree = sample();
        assert_eq!(
            tree.insert(Some("nope"), vec![TreeNode::leaf("x")], None)
                .unwrap_err(),
            CollectionError::UnknownNode("nope".to_owned())
        );
        assert!(
            tree.insert(None, vec![TreeNode::leaf("src")], None)
                .is_err()
        );
        assert!(matches!(
            tree.insert(None, vec![TreeNode::leaf("x")], Some(4)),
            Err(CollectionError::IndexOutOfRange { index: 4, .. })
        ));
    }

    #[test]
    fn removes_nodes_and_their_descendants() {
        let tree = sample();
        let removed = tree.remove(["src/widgets", "unknown"]);
        assert!(!removed.has("src/widgets/button.dart"));
        assert!(removed.has("src/app.dart"));
        assert!(tree.has("src/widgets"));
    }

    #[test]
    fn moves_nodes_in_tree_order() {
        let tree = sample();
        let moved = tree
            .move_nodes(["README.md", "src/app.dart"], Some("test"), None)
            .expect("moves");
        assert_eq!(
            values(moved.children_of("test")),
            ["test/app_test.dart", "src/app.dart", "README.md"]
        );
        assert_eq!(moved.parent_of("README.md"), Some("test"));
    }

    #[test]
    fn moving_into_a_moved_node_or_an_unknown_node_is_refused() {
        let tree = sample();
        assert!(matches!(
            tree.move_nodes(["src"], Some("src/widgets"), None),
            Err(CollectionError::InvalidMove(_))
        ));
        assert!(matches!(
            tree.move_nodes(["src"], Some("src"), None),
            Err(CollectionError::InvalidMove(_))
        ));
        assert!(matches!(
            tree.move_nodes(["README.md"], Some("nope"), None),
            Err(CollectionError::UnknownNode(_))
        ));
    }

    #[test]
    fn moving_a_branch_brings_its_descendants() {
        let tree = sample();
        let moved = tree
            .move_nodes(["src/widgets", "src/widgets/button.dart"], None, Some(0))
            .expect("moves");
        assert_eq!(moved.parent_of("src/widgets"), None);
        assert_eq!(
            moved.parent_of("src/widgets/button.dart"),
            Some("src/widgets")
        );
        assert_eq!(moved.roots()[0].value, "src/widgets");
    }

    #[test]
    fn replaces_a_node_in_a_new_tree() {
        let tree = sample();
        let replaced = tree
            .replace(
                "README.md",
                TreeNode::leaf("README.md").with_label("Read me"),
            )
            .expect("replaces");
        assert_eq!(
            replaced.find("README.md").map(TreeNode::label),
            Some("Read me")
        );
        assert_eq!(
            tree.find("README.md").map(TreeNode::label),
            Some("README.md")
        );
        assert!(tree.replace("README.md", TreeNode::leaf("src")).is_err());
        assert_eq!(
            tree.replace("missing", TreeNode::leaf("z")).expect("ok"),
            tree
        );
    }

    #[test]
    fn nodes_carry_a_payload() {
        let node = TreeNode::leaf("a").with_data(7_u32);
        assert_eq!(node.data, Some(7));
        assert_eq!(node.label(), "a");
    }
}
