use std::collections::HashSet;

use native::journey_bitmap::{Block, JourneyBitmap, Tile};
use protobuf::Message;
use rand::Rng;

// 填充随机位置的函数
fn fill_random_positions(journey_bitmap: &mut JourneyBitmap, n: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..n {
        if i % 10000 == 0 {
            println!("填充第{}个,剩余{}个", i, n - i);
        }
        let tiles_key = (rng.gen(), rng.gen());
        let blocks_key = (rng.gen(), rng.gen());
        let tile = journey_bitmap
            .tiles
            .entry(tiles_key)
            .or_insert(Tile::new(tiles_key.0, tiles_key.1));
        let block: &mut Block = tile.blocks.entry(blocks_key).or_insert(create_block());
    }
}

// 生成 20%填充block
fn create_block() -> Block {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;
    const FILL_PERCENTAGE: f32 = 0.2;
    let mut rng = rand::thread_rng();
    let mut data: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    // 计算要填充的像素数量
    let fill_pixel_count = ((WIDTH * HEIGHT) as f32 * FILL_PERCENTAGE) as usize;
    // 随机填充 1 的像素，确保不重复
    let mut selected_points = HashSet::new();
    while selected_points.len() < fill_pixel_count {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        selected_points.insert((x, y));
    }
    for (x, y) in selected_points {
        data[y][x] = 1;
    }
    // 将二维数组转换为一维数组
    let flat_data: Vec<u8> = data.iter().flatten().copied().collect();
    // 填充为大小为512的数组
    let mut array512: [u8; 512] = [0; 512];
    for (i, &pixel) in flat_data.iter().enumerate().take(512) {
        array512[i] = pixel;
    }
    let block = Block::new_with_data(rng.gen(), rng.gen(), array512);
    return block;
}

#[test]
fn test1() {
    // 计算总元素数量和需要填充的元素数量
    let total_elements: u64 = 512 * 512 * 128 * 128;
    // let elements_to_fill: usize = (total_elements as f64 * 0.01) as usize;
    // 内存占用过多 测试500w个
    let elements_to_fill: usize = 500 * 10000 as usize;
    let mut journey_bitmap = JourneyBitmap::new();
    println!("blocks 的数量: {}", total_elements);
    println!("需要填充blocks 的数量: {}", elements_to_fill);
    fill_random_positions(&mut journey_bitmap, elements_to_fill);
    let proto = journey_bitmap.to_proto();
    println!("proto占用{}bytes", &proto.compute_size());
    let buf = &proto.write_to_bytes().unwrap();
    let buf = zstd::encode_all(buf.as_slice(), 3);
    println!("压缩后占用{}bytes", &buf.unwrap().len());
}
