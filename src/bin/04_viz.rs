use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;

advent_of_code::solution!(4);

// For visualization
const GIF_PATH: &str = "./media/day04_part2.gif";
const COLOR_TRUE: Rgb<u8> = Rgb([173, 216, 230]); // Pastel Blue
const COLOR_FALSE: Rgb<u8> = Rgb([0, 0, 0]); // Black
const COLOR_UPDATED: Rgb<u8> = Rgb([225, 182, 193]); // Pastel Pink
const UPSCALE: u32 = 10;
const NR_REMOVED_PER_FRAME: usize = 20;

const MAX_COUNT: u8 = 4;
const DIRS: [(i32, i32); 8] = [
    (-1, 0),  // Left
    (1, 0),   // Right
    (0, -1),  // Up
    (0, 1),   // Down
    (-1, -1), // Up-Left
    (1, -1),  // Up-Right
    (-1, 1),  // Down-Left
    (1, 1),   // Down-Right
];

fn parse_input(input: &str) -> (Vec<bool>, (usize, usize)) {
    let n = input.split_once("\n").unwrap().0.len();
    let m = input.lines().filter(|l| !l.is_empty()).count();
    (
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect(),
        (n, m),
    )
}

#[inline(always)]
fn idx(x: i32, y: i32, n: i32) -> i32 {
    y * n + x
}

#[inline(always)]
fn reverse_idx(idx: usize, n: usize) -> (usize, usize) {
    (idx % n, idx / n)
}

#[inline(always)]
fn neighbours(grid: &[bool], x: usize, y: usize, n: i32, m: i32) -> u8 {
    let mut count = 0;
    for (dx, dy) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < n && ny >= 0 && ny < m {
            count += grid[idx(nx, ny, n) as usize] as u8;
        }
    }
    count
}

/// Helper used for generating GIF frames in visualization
fn generate_frame(
    grid: &[bool],
    n: usize,
    m: usize,
    updated: &[usize],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = RgbImage::new(n as u32, m as u32);
    for i in 0..grid.len() {
        let (x, y) = reverse_idx(i, n);
        let pixel = if grid[i] { COLOR_TRUE } else { COLOR_FALSE };
        img.put_pixel(x as u32, y as u32, pixel);
    }
    for &i in updated {
        let (x, y) = reverse_idx(i, n);
        let pixel = COLOR_UPDATED;
        img.put_pixel(x as u32, y as u32, pixel);
    }
    let upscaled = image::imageops::resize(
        &img,
        n as u32 * UPSCALE,
        m as u32 * UPSCALE,
        image::imageops::FilterType::Nearest,
    );
    upscaled
}

pub fn part_one(_input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, (n, m)) = parse_input(input);
    let (n_i32, m_i32) = (n as i32, m as i32);
    let mut total = 0;

    let mut frames = Vec::new();
    let mut updated = vec![];
    // Start state for image
    let img = generate_frame(&grid, n, m, &updated);
    frames.push(img.clone());

    // Vector of neighbour counts
    let mut counts = grid
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if !c {
                0
            } else {
                let (x, y) = reverse_idx(i, n);
                neighbours(&grid, x, y, n_i32, m_i32)
            }
        })
        .collect::<Vec<u8>>();

    // Vector of indices of next @ values to check
    let mut queue = grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if !c {
                None
            } else {
                if counts[i] < MAX_COUNT { Some(i) } else { None }
            }
        })
        .collect::<Vec<usize>>();

    while let Some(i) = queue.pop() {
        total += 1;
        grid[i] = false;
        updated.push(i);
        if updated.len() >= NR_REMOVED_PER_FRAME {
            let img = generate_frame(&grid, n, m, &updated);
            frames.push(img);
            updated.clear();
        }
        if counts[i] > 0 {
            // Update neighbour counts
            let (x, y) = reverse_idx(i, n);
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n_i32 && ny >= 0 && ny < m_i32 {
                    let ni = idx(nx, ny, n_i32) as usize;
                    counts[ni] = counts[ni].saturating_sub(1);
                    // Branching here on if counts[ni] > 0 is slower.
                    // Check == MAX_COUNT - 1, not <= MAX_COUNT - 1 as then you duplicate elements in queue
                    if counts[ni] == MAX_COUNT - 1 {
                        queue.push(ni);
                    }
                }
            }
        }
    }

    // End state for image
    if !updated.is_empty() {
        // Add last frame
        let img = generate_frame(&grid, n, m, &updated);
        frames.push(img.clone());
        updated.clear();
    }
    let img = generate_frame(&grid, n, m, &updated);
    frames.push(img.clone());
    // --- Write frames to GIF ---
    let mut image = File::create(GIF_PATH).unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        n as u16 * UPSCALE as u16,
        m as u16 * UPSCALE as u16,
        &[0, 255],
    )
    .unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let n_frames = frames.len();
    for (i, img) in frames.into_iter().enumerate() {
        let raw = img.into_raw();
        let mut frame = Frame::from_rgb(n as u16 * UPSCALE as u16, m as u16 * UPSCALE as u16, &raw);
        if i == 0 || i == n_frames - 1 {
            frame.delay = 200; // 1000ms per frame
        } else {
            frame.delay = 5; // 50ms per frame
        }
        encoder.write_frame(&frame).unwrap();
    }
    Some(total)
}
