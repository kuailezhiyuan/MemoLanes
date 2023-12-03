use native::journey_bitmap::{self, Block, JourneyBitmap, Tile};
use protobuf::Message;
use rand::Rng;

const TILE_WIDTH: i32 = 512;
const BLOCK_WIDTH: i32 = 128;
const TILE_RATE: f64 = 0.04;
const BLOCK_RATE: f64 = 0.02;
const BITMAP_RATE: f64 = 0.4;
const TOTAL_RATE: f64 = TILE_RATE * BLOCK_RATE * BITMAP_RATE;

// 生成 20%填充block
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

// cargo test test1 --release -- --nocapture
#[test]
fn test1() {
    println!(
        "tile_rate: {:.1}%, block_rate: {:.1}%, bitmap_rate: {:.1}%",
        TILE_RATE * 100.,
        BLOCK_RATE * 100.,
        BITMAP_RATE * 100.
    );
    println!("total_rate: {:.3}%", TOTAL_RATE * 100.);

    let journey_bitmap = generate_journey_bitmap();

    let proto = journey_bitmap.to_proto();
    println!("proto占用{}bytes", &proto.compute_size());
    let buf = &proto.write_to_bytes().unwrap();
    let buf = zstd::encode_all(buf.as_slice(), 3);
    println!("压缩后占用{}bytes", &buf.unwrap().len());
}
