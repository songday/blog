use crate::util::num;

pub struct NumberImage {
    pub data: &'static [u8],
}

include!(concat!("number_image.rs"));

pub fn rand_group_number_image(num_pos: usize) -> &'static NumberImage {
    let group = num::rand_number(0, 4);
    &NUMBER_IMAGE_GROUPS[group][num_pos]
}
