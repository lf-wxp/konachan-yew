use fake::{
  faker::name::raw::FirstName,
  locales::{EN, ZH_CN},
  Dummy, Fake,
};
pub struct RandomName;
impl Dummy<RandomName> for String {
  fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &RandomName, rng: &mut R) -> Self {
    let x = rng.gen_range(0..=1);
    if x % 2 == 0 {
      FirstName(ZH_CN).fake()
    } else {
      FirstName(EN).fake()
    }
  }
}

