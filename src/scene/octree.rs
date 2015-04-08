// Copyright (c) 2015, Sam Payson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// Until I'm done w/ the design.
#![allow(dead_code)]

use core::nonzero::NonZero;

use math;
use math::{BoundingCube,Octant,S0,SX,SY,SZ};
use units::*;

/// An EntryID identifies an object which has been inserted into a `LooseOctree`.
pub type EntryID = u32;

// A NodeID is a bitwise negation of an index into a LooseOctree's `nodes` field. Hence, the "null"
// node corresponds to the node at index u32::MAX.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct NodeID(NonZero<u32>);

impl NodeID {
    fn as_index(self) -> usize { !*self.0 as usize }
}

/// A Loose Octree is a data structure for maintaining the locations of objects in 3D space.
pub struct LooseOctree<T> {
    // The node which is the root of the tree.
    root: NodeID,

    // The nodes of the octree.
    nodes: Vec<Node>,

    // The nodes hold indices into this array.
    entries: Vec<Entry<T>>,

    // The smallest dimension that any segment of the octree may have.
    min_dist: Meters,
}

impl<T> LooseOctree<T> {
    /// Create a new octree with an initial root node containing the given bounding box, and which
    /// will never produce a node less than half of `min` meters to a side.
    pub fn new(initial: BoundingCube, min: Meters) -> LooseOctree<T> {
        let mut octree = LooseOctree {
            root:     NodeID(unsafe { NonZero::new(!0) }),
            nodes:    vec![],
            entries:  vec![],
            min_dist: min,
        };

        octree.root = octree.new_node(Node {
            bcube:    initial,
            octant:   S0,
            parent:   None,
            children: [None; 8],
            contents: vec![],
        });

        octree
     }

    // Create a new node within the tree.
    #[inline] fn new_node(&mut self, n: Node) -> NodeID {
        let idx = self.nodes.len() as u32;

        self.nodes.push(n);

        // !idx is non-zero as long as self.nodes.len() is less than u32::MAX. Pretty safe bet, or
        // else we're using 120GB of RAM on the tree's nodes alone =].
        NodeID(unsafe { NonZero::new(!idx) })
    }

    #[inline] fn node_by_id(&self, id: NodeID) -> &Node {
        &self.nodes[id.as_index()]
    }

    #[inline] fn node_by_id_mut(&mut self, id: NodeID) -> &mut Node {
        &mut self.nodes[id.as_index()]
    }

    // Free a node from the tree. This re-arranges the backing array by swapping the last element
    // with the element being freed. This requires some fixup work (to correct the trees internal
    // pointers) which makes up the majority of this function.
    fn free_node(&mut self, id: NodeID) {
        debug_assert!(id.as_index() < self.nodes.len());
        debug_assert!(id != self.root);
        debug_assert!(self.nodes.len() > 0);

        // This is a special case, if the node is the last element then there's no fixup required.
        if id.as_index() == self.nodes.len() - 1 {
            self.nodes.pop();
            return
        }

        // Remove the last element
        let last = self.nodes.pop().unwrap();

        // Fixup references to `last` so that they reference the location of the element we're about
        // to replace.

        if let Some(parent_id) = last.parent {
            self.nodes[parent_id.as_index()].children[last.octant.bits() as usize] = Some(id);
        }

        for &child in last.children.iter() {
            if let Some(child_id) = child {
                self.nodes[child_id.as_index()].parent = Some(id);
            }
        }

        for &ent_id in last.contents.iter() {
            self.entries[ent_id as usize].node = id;
        }

        // Overwrite the "freed" node with the element we removed from the end.
        self.nodes[id.as_index()] = last;
    }

    /// Insert an object into the octree.
    pub fn insert(&mut self, val: T, bcube: BoundingCube) -> EntryID {
        let ent_id = self.entries.len() as EntryID;

        // It's quite frustrating that this needs to be here, rather than in the invocation of
        // self.get_node().
        let root = self.root;

        let node = self.get_node(root, bcube);

        self.entries.push(Entry { bcube: bcube, val: val, node: node });

        ent_id
    }

    /// Modify the location of an existing entry in the tree.
    pub fn adjust(&mut self, ent_id: EntryID, bcube: BoundingCube) {
        let current_node = self.entries[ent_id as usize].node;

        // Get the node which *should* contain this entry.
        let new_node = self.get_node(current_node, bcube);

        if new_node != current_node {
            self.node_by_id_mut(current_node).contents.retain(|&x| { x != ent_id });
            self.maybe_free(current_node);

            self.node_by_id_mut(new_node).contents.push(ent_id);
            self.entries[ent_id as usize].node = new_node;
        }
    }

    // Release a node if it has no contents and no children. Otherwise leave it unaffected.
    fn maybe_free(&mut self, id: NodeID) {
        // We don't ever free the root node, even if it's empty.
        if let Some(parent_id) = self.node_by_id(id).parent {

            // Only free the node if the node has no children and no contents.
            if self.node_by_id(id).contents.is_empty()
                && !self.node_by_id(id).children.iter().any(|&x| x.is_some()) {

                self.free_node(id);

                self.maybe_free(parent_id);
            }
        }
    }

    // Return the node which should contain the given bounding box. Begin the search at the node
    // referred to by `id`. This routine will allocate new nodes if necessary, and may even create a
    // new root node.
    fn get_node(&mut self, id: NodeID, bcube: BoundingCube) -> NodeID {
        let node_bcube = self.node_by_id(id).bcube;

        // Which octant of `id` contains `bcube`?
        match node_bcube.octant(bcube.center) {
            // Ok, the center of `bcube` is inside of this node, there are three posibilities:
            //
            //  1. `bcube` is too small for the node, and needs to go in a child node.
            //  2. `bcube` is too big for this node, and needs to go in a parent node.
            //  3. `bcube` "fits" in this node, and so we just return this node's ID.
            Some(octant) => if node_bcube.half_edge / Meters(2.0) > bcube.half_edge
                            && node_bcube.half_edge < self.min_dist {
                // Case 1: recurse on a child node.
                let child = self.get_child(id, octant);

                self.get_node(child, bcube)
            } else if node_bcube.half_edge < bcube.half_edge {
                // Case 2: Recurse on the parent node, creating one if it doesn't exist.
                let parent = match self.node_by_id(id).parent {
                    Some(parent) => parent,
                    None         => self.make_parent_toward(id, bcube.center),
                };

                self.get_node(parent, bcube)
            } else {
                id
            },

            None => {
                let parent = match self.node_by_id(id).parent {
                    Some(parent) => parent,
                    None         => self.make_parent_toward(id, bcube.center),
                };

                self.get_node(parent, bcube)
            }
        }
    }

    // Get a child node of `id`, creating one if it doesn't already exist.
    fn get_child(&mut self, id: NodeID, octant: Octant) -> NodeID {
        if let Some(child) = self.node_by_id(id).children[octant.bits() as usize] {
            // Child already exists, just return it.
            return child;
        }

        // Okay, we've gotta construct a child node. The math is straightforward.

        let old_bcube = self.node_by_id(id).bcube;

        let new_center = old_bcube.center
                       + octant.as_vector().scaled(old_bcube.half_edge / Meters(2.0));

        let child = self.new_node(Node {
            bcube: BoundingCube {
                center:    new_center,
                half_edge: old_bcube.half_edge * Meters(0.5),
            },
            octant:   octant,
            parent:   Some(id),
            children: [None; 8],
            contents: vec![],
        });

        self.node_by_id_mut(id).children[octant.bits() as usize] = Some(child);

        child
    }

    // Create a parent node of `id` which comes closer to containing `v` than `id` itself (though it
    // might not actually end up containing `v`).
    fn make_parent_toward(&mut self, id: NodeID, v: math::Vec3<Meters>) -> NodeID {
        let old_bcube = self.node_by_id(id).bcube;
        let diff      = v - old_bcube.center;

        // Which octant of the parent will be `id`?
        let octant = if diff.x < Meters(0.0) { S0 } else { SX }
                   | if diff.y < Meters(0.0) { S0 } else { SY }
                   | if diff.z < Meters(0.0) { S0 } else { SZ };

        let new_center = old_bcube.center + octant.as_vector().scaled(-old_bcube.half_edge);

        let node = self.new_node(Node {
            bcube: BoundingCube {
                center:    new_center,
                half_edge: old_bcube.half_edge * Meters(2.0),
            },
            octant:   S0,
            parent:   None, // This is a top-level node.
            children: [None; 8],
            contents: vec![],
        });

        self.node_by_id_mut(node).children[octant.bits() as usize] = Some(id);

        node
    }
}

struct Entry<T> {
    // A Cube which bounds this entry
    bcube: BoundingCube,

    // The node which currently contains this entry
    node: NodeID,

    // The item at this location.
    val: T,
}

struct Node {
    // Cube in space which this node represents. Note that, because this is a Loose Octree, geometry
    // contained in this node is only guaranteed to inside a bounding cube with a `half_edge`
    // *twice* that of the `bcube` field.
    bcube: BoundingCube,

    // Which octant of the parent node contains this node. This will be S0 for the root.
    octant: Octant,

    // This node's parent.
    parent: Option<NodeID>,

    // The nodes which are contained inside of this one.
    children: [Option<NodeID>; 8],

    // Indices into the `entries` field of the Octree. This field has the potential to be a
    // bottleneck, since we're going to do lots of naive linear search on it.
    contents: Vec<EntryID>,
}
