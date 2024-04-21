use core::mem;
use nom_derive::*;

use std::io::{BufWriter, Cursor, IoSliceMut, Read, Write};

use binrw::{
    BinRead,  // trait for reading
    BinWrite, // trait for writing
    BinWriterExt,
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use deku::{ctx::Limit, prelude::*, DekuRead, DekuWrite};

#[derive(Nom, DekuRead, BinRead, BinWrite, DekuWrite, PartialEq, Eq, Debug, Default)]
#[deku(endian = "little")]
#[nom(LittleEndian)]
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

    fn from_bytes_custom<T: Read>(reader: &mut T) -> std::io::Result<Self> {
        let mut magic = [0; 4];
        reader.read_exact(&mut magic)?;
        let magic = u32::from_le_bytes(magic);

        let mut inode_count = [0; 4];
        reader.read_exact(&mut inode_count)?;
        let inode_count = u32::from_le_bytes(inode_count);

        let mut mod_time = [0; 4];
        reader.read_exact(&mut mod_time)?;
        let mod_time = u32::from_le_bytes(mod_time);

        let mut block_size = [0; 4];
        reader.read_exact(&mut block_size)?;
        let block_size = u32::from_le_bytes(block_size);

        let mut frag_count = [0; 4];
        reader.read_exact(&mut frag_count)?;
        let frag_count = u32::from_le_bytes(frag_count);

        let mut compressor = [0; 2];
        reader.read_exact(&mut compressor)?;
        let compressor = u16::from_le_bytes(compressor);

        let mut block_log = [0; 2];
        reader.read_exact(&mut block_log)?;
        let block_log = u16::from_le_bytes(block_log);

        let mut flags = [0; 2];
        reader.read_exact(&mut flags)?;
        let flags = u16::from_le_bytes(flags);

        let mut id_count = [0; 2];
        reader.read_exact(&mut id_count)?;
        let id_count = u16::from_le_bytes(id_count);

        let mut version_major = [0; 2];
        reader.read_exact(&mut version_major)?;
        let version_major = u16::from_le_bytes(version_major);

        let mut version_minor = [0; 2];
        reader.read_exact(&mut version_minor)?;
        let version_minor = u16::from_le_bytes(version_minor);

        let mut root_inode = [0; 8];
        reader.read_exact(&mut root_inode)?;
        let root_inode = u64::from_le_bytes(root_inode);

        let mut bytes_used = [0; 8];
        reader.read_exact(&mut bytes_used)?;
        let bytes_used = u64::from_le_bytes(bytes_used);

        let mut id_table = [0; 8];
        reader.read_exact(&mut id_table)?;
        let id_table = u64::from_le_bytes(id_table);

        let mut xattr_table = [0; 8];
        reader.read_exact(&mut xattr_table)?;
        let xattr_table = u64::from_le_bytes(xattr_table);

        let mut inode_table = [0; 8];
        reader.read_exact(&mut inode_table)?;
        let inode_table = u64::from_le_bytes(inode_table);

        let mut dir_table = [0; 8];
        reader.read_exact(&mut dir_table)?;
        let dir_table = u64::from_le_bytes(dir_table);

        let mut frag_table = [0; 8];
        reader.read_exact(&mut frag_table)?;
        let frag_table = u64::from_le_bytes(frag_table);

        let mut export_table = [0; 8];
        reader.read_exact(&mut export_table)?;
        let export_table = u64::from_le_bytes(export_table);

        Ok(Self {
            magic,
            inode_count,
            mod_time,
            block_size,
            frag_count,
            compressor,
            block_log,
            flags,
            id_count,
            version_major,
            version_minor,
            root_inode,
            bytes_used,
            id_table,
            xattr_table,
            inode_table,
            dir_table,
            frag_table,
            export_table,
        })
    }
}

fn bench_serialise(c: &mut Criterion) {
    let input: Vec<_> = (0..10_0000).map(|i| SuperBlock::default()).collect();
    let mut group = c.benchmark_group("Serialize");
    group.bench_function("deku", |b| {
        b.iter(|| {
            // Deku doesn't need `Seek`, so no Cursor
            let out_buf = vec![];
            let mut writer = deku::writer::Writer::new(out_buf);
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

    group.bench_function("nom-derive", |b| {
        b.iter_batched(
            || cursor.clone(),
            |mut reader| {
                let mut a = SuperBlock::default();
                for _ in 0..10_0000 {
                    let mut buf = [0; mem::size_of::<SuperBlock>()];
                    reader.read_exact(&mut buf).unwrap();
                    a = SuperBlock::parse(&mut buf).unwrap().1;
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
                    a = SuperBlock::from_bytes_custom(&mut reader).unwrap();
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
