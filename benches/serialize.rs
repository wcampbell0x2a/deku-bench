use core::mem;
use std::io::{BufWriter, Cursor, IoSliceMut, Read, Write};

use binrw::{
    BinRead,  // trait for reading
    BinWrite, // trait for writing
    BinWriterExt,
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use deku::{ctx::Limit, prelude::*, DekuRead, DekuWrite};

#[derive(DekuRead, BinRead, BinWrite, DekuWrite, PartialEq, Eq, Debug, Default)]
#[deku(endian = "little")]
pub struct SuperBlock {
    magic: u32,
    inode_count: u32,
    mod_time: u32,
    block_size: u32,
    frag_count: u32,
    compressor: u16,
    block_log: u16,
    flags: u16,
    id_count: u16,
    version_major: u16,
    version_minor: u16,
    root_inode: u64,
    bytes_used: u64,
    id_table: u64,
    xattr_table: u64,
    inode_table: u64,
    dir_table: u64,
    frag_table: u64,
    export_table: u64,
}

// writing these made me want to use deku :^)
impl SuperBlock {
    fn to_bytes_custom<T: Write>(&self, writer: &mut T) {
        writer.write(&self.magic.to_le_bytes()).unwrap();
        writer.write(&self.inode_count.to_le_bytes()).unwrap();
        writer.write(&self.mod_time.to_le_bytes()).unwrap();
        writer.write(&self.block_size.to_le_bytes()).unwrap();
        writer.write(&self.frag_count.to_le_bytes()).unwrap();
        writer.write(&self.compressor.to_le_bytes()).unwrap();
        writer.write(&self.block_log.to_le_bytes()).unwrap();
        writer.write(&self.flags.to_le_bytes()).unwrap();
        writer.write(&self.id_count.to_le_bytes()).unwrap();
        writer.write(&self.version_major.to_le_bytes()).unwrap();
        writer.write(&self.version_minor.to_le_bytes()).unwrap();
        writer.write(&self.root_inode.to_le_bytes()).unwrap();
        writer.write(&self.bytes_used.to_le_bytes()).unwrap();
        writer.write(&self.id_table.to_le_bytes()).unwrap();
        writer.write(&self.xattr_table.to_le_bytes()).unwrap();
        writer.write(&self.inode_table.to_le_bytes()).unwrap();
        writer.write(&self.dir_table.to_le_bytes()).unwrap();
        writer.write(&self.frag_table.to_le_bytes()).unwrap();
        writer.write(&self.export_table.to_le_bytes()).unwrap();
    }

    fn from_bytes_custom<T: Read>(reader: &mut T) -> Self {
        let (
            mut a,
            mut b,
            mut c,
            mut d,
            mut e,
            mut f,
            mut g,
            mut h,
            mut i,
            mut j,
            mut k,
            mut l,
            mut m,
            mut n,
            mut o,
            mut p,
            mut q,
            mut r,
            mut s,
        ) = (
            [0; mem::size_of::<u32>()], // magic
            [0; mem::size_of::<u32>()], // inode_count
            [0; mem::size_of::<u32>()], // mod_time
            [0; mem::size_of::<u32>()], // block_size
            [0; mem::size_of::<u32>()], // frag_count
            [0; mem::size_of::<u16>()], // compressor
            [0; mem::size_of::<u16>()], // block_log
            [0; mem::size_of::<u16>()], // flags
            [0; mem::size_of::<u16>()], // id_count
            [0; mem::size_of::<u16>()], // version_major
            [0; mem::size_of::<u16>()], // version_minor
            [0; mem::size_of::<u64>()], // root_inode
            [0; mem::size_of::<u64>()], // bytes_used
            [0; mem::size_of::<u64>()], // id_table
            [0; mem::size_of::<u64>()], // xattr_table
            [0; mem::size_of::<u64>()], // inode_table
            [0; mem::size_of::<u64>()], // dir_table
            [0; mem::size_of::<u64>()], // frag_table
            [0; mem::size_of::<u64>()], // export_table
        );
        let mut buf = [
            IoSliceMut::new(&mut a),
            IoSliceMut::new(&mut b),
            IoSliceMut::new(&mut c),
            IoSliceMut::new(&mut d),
            IoSliceMut::new(&mut e),
            IoSliceMut::new(&mut f),
            IoSliceMut::new(&mut g),
            IoSliceMut::new(&mut h),
            IoSliceMut::new(&mut i),
            IoSliceMut::new(&mut j),
            IoSliceMut::new(&mut k),
            IoSliceMut::new(&mut l),
            IoSliceMut::new(&mut m),
            IoSliceMut::new(&mut n),
            IoSliceMut::new(&mut o),
            IoSliceMut::new(&mut p),
            IoSliceMut::new(&mut q),
            IoSliceMut::new(&mut r),
            IoSliceMut::new(&mut s),
        ];
        let len = reader.read_vectored(&mut buf).unwrap();
        assert_eq!(len, 96);
        Self {
            magic: u32::from_le_bytes(a),
            inode_count: u32::from_le_bytes(b),
            mod_time: u32::from_le_bytes(c),
            block_size: u32::from_le_bytes(d),
            frag_count: u32::from_le_bytes(e),
            compressor: u16::from_le_bytes(f),
            block_log: u16::from_le_bytes(g),
            flags: u16::from_le_bytes(h),
            id_count: u16::from_le_bytes(i),
            version_major: u16::from_le_bytes(j),
            version_minor: u16::from_le_bytes(k),
            root_inode: u64::from_le_bytes(l),
            bytes_used: u64::from_le_bytes(m),
            id_table: u64::from_le_bytes(n),
            xattr_table: u64::from_le_bytes(o),
            inode_table: u64::from_le_bytes(p),
            dir_table: u64::from_le_bytes(q),
            frag_table: u64::from_le_bytes(r),
            export_table: u64::from_le_bytes(s),
        }
    }
}

fn bench_serialise(c: &mut Criterion) {
    let input: Vec<_> = (0..10_0000).map(|i| SuperBlock::default()).collect();
    let mut group = c.benchmark_group("Serialize");
    group.bench_function("deku", |b| {
        b.iter(|| {
            let out_buf = vec![];
            let mut writer = BufWriter::new(out_buf);
            let mut writer = deku::writer::Writer::new(&mut writer);
            for i in input.iter() {
                i.to_writer(&mut writer, ()).unwrap()
            }
        })
    });
    group.bench_function("binrw", |b| {
        b.iter(|| {
            let mut buf = Cursor::new(Vec::new());
            buf.write_le(&input).unwrap();
        })
    });
    group.bench_function("custom", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            for i in input.iter() {
                i.to_bytes_custom(&mut buf)
            }
        })
    });
    group.finish();
    let mut buf = Cursor::new(Vec::new());
    buf.write_le(&input).unwrap();
    let binrw = buf.into_inner();
    assert_eq!(
        binrw,
        input
            .iter()
            .flat_map(|x| x.to_bytes().unwrap().into_iter())
            .collect::<Vec<_>>()
    );
    let mut custom = Vec::new();
    for i in input.iter() {
        i.to_bytes_custom(&mut custom)
    }
    assert_eq!(binrw, custom);
}

// TODO: Add BufReader to these all
fn bench_deserialise(c: &mut Criterion) {
    let input: Vec<_> = (0..10_0000)
        .map(|i| SuperBlock {
            magic: i,
            ..SuperBlock::default()
        })
        .collect();
    let mut custom = Vec::new();
    for i in input.iter() {
        i.to_bytes_custom(&mut custom)
    }

    let mut group = c.benchmark_group("Deserialize");
    let cursor = std::io::Cursor::new(custom.clone());
    group.bench_function("deku", |b| {
        b.iter_batched(
            || cursor.clone(),
            |mut reader| {
                let mut a = SuperBlock::default();
                for _ in 0..10_0000 {
                    a = SuperBlock::from_reader((&mut reader, 0)).unwrap().1;
                }
                assert_eq!(a.magic, 10_0000 - 1);
            },
            BatchSize::SmallInput,
        );
    });

    let cursor = std::io::Cursor::new(custom.clone());
    group.bench_function("custom", |b| {
        b.iter_batched(
            || cursor.clone(),
            |mut reader| {
                let mut a = SuperBlock::default();
                for _ in 0..10_0000 {
                    a = SuperBlock::from_bytes_custom(&mut reader);
                }
                assert_eq!(a.magic, 10_0000 - 1);
            },
            BatchSize::SmallInput,
        );
    });

    // binrw::io::BufReader made this slower?
    let binrw = binrw::io::Cursor::new(custom.clone());
    group.bench_function("binrw", |b| {
        b.iter_batched(
            || binrw.clone(),
            |mut binrw_input| {
                let mut a = SuperBlock::default();
                for i in 0..10_0000 {
                    a = SuperBlock::read_le(&mut binrw_input).unwrap();
                }
                assert_eq!(a.magic, 10_0000 - 1);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, bench_serialise, bench_deserialise);
criterion_main!(benches);
