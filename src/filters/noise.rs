use filters::Filter;
use images::{Image, Pixl};
use rand::{thread_rng, Rng};

pub struct Noise {
    prob: f32,
}

impl Noise {
    pub fn new(prob: f32) -> Noise {
        Noise { prob }
    }
}

impl Filter for Noise {
    fn apply(&self, i: &mut Image) {
        let mut rng = thread_rng();
        for y in 0..i.height() {
            for x in 0..i.width() {
                if rng.gen::<f32>() <= self.prob {
                    i.put_pixel(x, y, Pixl::black());
                }
            }
        }
    }
}
