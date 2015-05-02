// automatically generated by the FlatBuffers compiler, do not modify

use flatbuffers as fb;

#[derive(Clone,Copy)]
#[packed] pub struct AnimInstance {
    t_start: u64,
    duration: u64,
    id: u32,
    repeat: u8,
    __padding0: u8,
    __padding1: u16,
}

impl AnimInstance {
    pub fn new(t_start: u64, duration: u64, id: u32, repeat: bool) -> AnimInstance {
        AnimInstance {
            t_start: fb::Endian::to_le(t_start),
            duration: fb::Endian::to_le(duration),
            id: fb::Endian::to_le(id),
            repeat: fb::Endian::to_le(if repeat { 0u8 } else { 1u8 }),
            __padding0: 0,
            __padding1: 0,
        }
    }

    pub fn t_start(&self) -> u64 { fb::Endian::from_le(self.t_start) }

    pub fn duration(&self) -> u64 { fb::Endian::from_le(self.duration) }

    pub fn id(&self) -> u32 { fb::Endian::from_le(self.id) }

    pub fn repeat(&self) -> bool { fb::Endian::from_le(self.repeat) != 0 }

}

pub struct Anim {
    inner: fb::Table,
}

impl Anim {
    pub fn name(&self) -> Option<&fb::String> {
        self.inner.get_ref(4)
    }
    pub fn sheet(&self) -> Option<&fb::String> {
        self.inner.get_ref(6)
    }
    pub fn indices(&self) -> Option<&fb::Vector<u16>> {
        self.inner.get_ref(8)
    }
}

pub struct AnimBuilder<'x> {
    fbb:   &'x mut fb::FlatBufferBuilder,
    start: fb::UOffset,
}

impl<'x> AnimBuilder<'x> {
    pub fn new(fbb: &'x mut fb::FlatBufferBuilder) -> AnimBuilder<'x> {
        let start = fbb.start_table();
        AnimBuilder {
            fbb:   fbb,
            start: start,
        }
    }

    pub fn add_name(&mut self, name: fb::Offset<fb::String>) {
        self.fbb.add_offset(4, name)
    }

    pub fn add_sheet(&mut self, sheet: fb::Offset<fb::String>) {
        self.fbb.add_offset(6, sheet)
    }

    pub fn add_indices(&mut self, indices: fb::Offset<fb::Vector<u16>>) {
        self.fbb.add_offset(8, indices)
    }

    pub fn finish(&mut self) -> fb::Offset<Anim> {
        let o = fb::Offset::new(self.fbb.end_table(self.start, 3));
        // self.fbb.required(o, 4);  // name
        // self.fbb.required(o, 6);  // sheet
        // self.fbb.required(o, 8);  // indices
        o
    }
}
