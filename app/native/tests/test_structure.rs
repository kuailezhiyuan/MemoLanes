use get_size::GetSize;
use native::journey_bitmap::{self, Block, JourneyBitmap, Tile};
use protobuf::Message;
use rand::Rng;

const TILE_WIDTH: i32 = 512;
const BLOCK_WIDTH: i32 = 128;
const TILE_RATE: f64 = 0.06;
const BLOCK_RATE: f64 = 0.04;
const BITMAP_RATE: f64 = 0.2;
const TOTAL_RATE: f64 = TILE_RATE * BLOCK_RATE * BITMAP_RATE;

fn generate_block(x: u8, y: u8) -> Block {
    const SIZE: usize = 512;
    let mut rng = rand::thread_rng();
    let mut data = [0u8; SIZE];

    for byte in &mut data {
        for bit in 0..8 {
            if rng.gen_bool(TOTAL_RATE) {
                *byte |= 1 << bit;
            }
        }
    }
    return Block::new_with_data(x, y, data);
}

fn generate_tile(x: u16, y: u16) -> Tile {
    let mut rng = rand::thread_rng();
    let mut tile = Tile::new(x, y);

    let mut blocks_to_fill = (BLOCK_WIDTH as f64 * BLOCK_WIDTH as f64 * BLOCK_RATE) as i32;
    while blocks_to_fill > 0 {
        let x = rng.gen_range(0..BLOCK_WIDTH) as u8;
        let y = rng.gen_range(0..BLOCK_WIDTH) as u8;
        if !tile.blocks.contains_key(&(x, y)) {
            let _ = tile.blocks.insert((x, y), generate_block(x, y));

            blocks_to_fill -= 1;
        }
    }
    tile
}

fn generate_journey_bitmap() -> JourneyBitmap {
    let mut rng = rand::thread_rng();
    let mut journey_bitmap = JourneyBitmap::new();

    let mut tiles_to_fill = (TILE_WIDTH as f64 * TILE_WIDTH as f64 * TILE_RATE) as i32;
    while tiles_to_fill > 0 {
        if tiles_to_fill % 100 == 0 {
            println!("remaining tiles: {}", tiles_to_fill);
        }
        let x = rng.gen_range(0..TILE_WIDTH) as u16;
        let y = rng.gen_range(0..TILE_WIDTH) as u16;
        if !journey_bitmap.tiles.contains_key(&(x, y)) {
            let _ = journey_bitmap.tiles.insert((x, y), generate_tile(x, y));
            tiles_to_fill -= 1;
        }
    }
    journey_bitmap
}

fn print_size_human(name: &str, size_in_bytes: usize) {
    println!("{}: {:.2} MB", name, size_in_bytes as f64 / 1024. / 1024.)
}

// cargo test test_structure --release -- --nocapture
#[test]
fn test_structure() {
    println!(
        "tile_rate: {:.1}%, block_rate: {:.1}%, bitmap_rate: {:.1}%",
        TILE_RATE * 100.,
        BLOCK_RATE * 100.,
        BITMAP_RATE * 100.
    );
    println!("total_rate: {:.3}%", TOTAL_RATE * 100.);

    let journey_bitmap = generate_journey_bitmap();

    print_size_human("in memory size", journey_bitmap.get_heap_size());

    let proto = journey_bitmap.to_proto();
    {
        let buf = proto.write_to_bytes().unwrap();
        print_size_human("[whole] protobuf", buf.len());
        let buf = zstd::encode_all(buf.as_slice(), 3).unwrap();
        print_size_human("[whole] zstd", buf.len());
    }

    let mut total_tile_proto_size: usize = 0;
    let mut total_tile_zstd_size: usize = 0;
    for (tile) in &proto.tiles {
        let buf = tile.write_to_bytes().unwrap();
        total_tile_proto_size += buf.len();
        let buf = zstd::encode_all(buf.as_slice(), 3).unwrap();
        total_tile_zstd_size += buf.len();
    }

    print_size_human("[tile total] protobuf", total_tile_proto_size);
    print_size_human("[tile total] zstd", total_tile_zstd_size);

    print_size_human(
        "[tile avg] protobuf",
        total_tile_proto_size / proto.tiles.len(),
    );
    print_size_human("[tile avg] zstd", total_tile_zstd_size / proto.tiles.len());
}
