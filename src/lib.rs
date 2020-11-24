#![feature(external_doc)]
use sd_rs::SD;
use ta_common::traits::Indicator;
#[doc(include = "../README.md")]
pub struct BBand {
    period: u32,
    scale: u32,
    sd: SD,
}

impl BBand {
    pub fn new(period: u32, scale: u32) -> BBand {
        Self {
            period,
            scale,
            sd: SD::new(period),
        }
    }
}

impl Indicator<f64, Option<[f64; 3]>> for BBand {
    fn next(&mut self, input: f64) -> Option<[f64; 3]> {
        let sd = self.sd.next(input);
        match sd {
            None => None,
            Some(sd) => {
                let bbm = self.sd.get_current_sma();
                let bbu = bbm + sd * self.scale as f64;
                let bbl = bbm - sd * self.scale as f64;
                Some([bbl, bbm, bbu])
            }
        }
    }

    fn reset(&mut self) {
        self.sd.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::BBand;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut bband = BBand::new(5, 2);
        assert_eq!(bband.next(81.59), None);
        assert_eq!(bband.next(81.06), None);
        assert_eq!(bband.next(82.87), None);
        assert_eq!(bband.next(83.00), None);
        assert_eq!(bband.next(83.61), Some([80.53004219456234, 82.426, 84.32195780543766]));
        assert_eq!(bband.next(83.15), Some([80.98714192465522, 82.73799999999999, 84.48885807534475]));
        assert_eq!(bband.next(82.84), Some([82.53334324225956, 83.09399999999998, 83.6546567577404]));
        assert_eq!(bband.next(83.99), Some([82.47198345169848, 83.31799999999998, 84.16401654830149]));
        assert_eq!(bband.next(84.55), Some([82.41775043895896, 83.62799999999999, 84.83824956104101]));
        assert_eq!(bband.next(84.36), Some([82.43520291927634, 83.77799999999999, 85.12079708072365]));
        assert_eq!(bband.next(85.53), Some([82.51133078296537, 84.25399999999998, 85.99666921703458]));
        assert_eq!(bband.next(86.54), Some([83.14261781363217, 84.99399999999997, 86.84538218636777]));
        assert_eq!(bband.next(86.89), Some([83.53648779144758, 85.57399999999997, 87.61151220855236]));
        assert_eq!(bband.next(87.77), Some([83.87032370204064, 86.21799999999996, 88.56567629795929]));
        assert_eq!(bband.next(87.29), Some([85.28887096259093, 86.80399999999996, 88.31912903740898]));
    }
}
