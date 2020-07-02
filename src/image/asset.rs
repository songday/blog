pub struct NumberImage {
    pub data: &'static [u8],
}

include!(concat!("number_image.rs"));

pub fn rand_group_number_image(num_pos: usize) -> &'static NumberImage {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let group: usize = rng.gen_range(0, 4);

    &NUMBER_IMAGE_GROUPS[group][num_pos]
}
