pub fn random_colour<R: rand::Rng + ?Sized>(rng: &mut R) -> [u8; 4] {
    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    let a: u8 = 255;

    [r, g, b, a]
}
