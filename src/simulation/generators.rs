use rand::prelude::*;
use rand_distr::StandardNormal;

pub fn pdf_standard_normal() -> f64 {
    let val: f64 = thread_rng().sample(StandardNormal);
    val
}
