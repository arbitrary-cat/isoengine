// automatically generated by the FlatBuffers compiler, do not modify

use flatbuffers as fb;

#[derive(Clone,Copy)]
#[repr(packed)] #[repr(C)] pub struct BoundingCube {
    center_x: f32,
    center_y: f32,
    center_z: f32,
    half_edge: f32,
}

impl BoundingCube {
    pub fn new(center_x: f32, center_y: f32, center_z: f32, half_edge: f32) -> BoundingCube {
        BoundingCube {
            center_x: fb::Endian::to_le(center_x),
            center_y: fb::Endian::to_le(center_y),
            center_z: fb::Endian::to_le(center_z),
            half_edge: fb::Endian::to_le(half_edge),
        }
    }

    pub fn center_x(&self) -> f32 { fb::Endian::from_le(self.center_x) }

    pub fn center_y(&self) -> f32 { fb::Endian::from_le(self.center_y) }

    pub fn center_z(&self) -> f32 { fb::Endian::from_le(self.center_z) }

    pub fn half_edge(&self) -> f32 { fb::Endian::from_le(self.half_edge) }

}

#[derive(Clone,Copy)]
#[repr(packed)] #[repr(C)] pub struct WorldLocation {
    bounds: BoundingCube,
}

impl WorldLocation {
    pub fn new(bounds: &BoundingCube) -> WorldLocation {
        WorldLocation {
            bounds: *bounds,
        }
    }

    pub fn bounds(&self) -> &BoundingCube { &self.bounds }

}

#[derive(Clone,Copy)]
#[repr(packed)] #[repr(C)] pub struct WorldRender {
    anim: ::grafix::anim::wire::AnimInstance,
}

impl WorldRender {
    pub fn new(anim: &::grafix::anim::wire::AnimInstance) -> WorldRender {
        WorldRender {
            anim: *anim,
        }
    }

    pub fn anim(&self) -> &::grafix::anim::wire::AnimInstance { &self.anim }

}

pub struct Entity {
    inner: fb::Table,
}

impl Entity {
    pub fn id(&self) -> u32 {
        self.inner.get_field(4, 0)
    }
    pub fn world_loc(&self) -> Option<&WorldLocation> {
        self.inner.get_struct(6)
    }
    pub fn world_ren(&self) -> Option<&WorldRender> {
        self.inner.get_struct(8)
    }
}

pub struct EntityBuilder<'x> {
    fbb:   &'x mut fb::FlatBufferBuilder,
    start: fb::UOffset,
}

impl<'x> EntityBuilder<'x> {
    pub fn new(fbb: &'x mut fb::FlatBufferBuilder) -> EntityBuilder<'x> {
        let start = fbb.start_table();
        EntityBuilder {
            fbb:   fbb,
            start: start,
        }
    }

    pub fn add_id(&mut self, id: u32) {
        self.fbb.add_scalar(4, id, 0)
    }

    pub fn add_world_loc(&mut self, world_loc: &WorldLocation) {
        self.fbb.add_struct(6, world_loc)
    }

    pub fn add_world_ren(&mut self, world_ren: &WorldRender) {
        self.fbb.add_struct(8, world_ren)
    }

    pub fn finish(&mut self) -> fb::Offset<Entity> {
        let o = fb::Offset::new(self.fbb.end_table(self.start, 3));
        o
    }
}

