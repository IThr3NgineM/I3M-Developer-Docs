use i3m::core::pool::Handle;
use i3m::graph::BaseSceneGraph;
use i3m::scene::graph::Graph;
use i3m::scene::node::Node;
use i3m::{
    core::{
        math::aabb::AxisAlignedBoundingBox,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    scene::{base::Base, node::NodeTrait},
};
use std::ops::{Deref, DerefMut};

// ANCHOR: custom_node
#[derive(Default, Clone, Reflect, Visit, Debug)]
pub struct CustomNode {
    base: Base,
}

impl Deref for CustomNode {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CustomNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl NodeTrait for CustomNode {
    i3m::impl_query_component!();

    fn local_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.base.local_bounding_box()
    }

    fn world_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.base.world_bounding_box()
    }

    fn id(&self) -> Uuid {
        // Provide unique id for serialization needs. It must be unique, use https://www.uuidgenerator.net/
        // to generate one.
        uuid!("f592e7f7-5e34-4043-9226-407c7457bb48")
    }
}
// ANCHOR_END: custom_node

// ANCHOR: add_custom_node
fn add_custom_node(graph: &mut Graph) -> Handle<Node> {
    graph.add_node(Node::new(CustomNode::default()))
}
// ANCHOR_END: add_custom_node
