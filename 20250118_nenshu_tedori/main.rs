const KENKORATE: f64 = 0.1158;
const NENKINRATE: f64 = 0.183;

struct RateUnit {
    lbound: isize,
    rbound: isize,
    rate_percent: isize,
    offset: isize,
}

impl RateUnit {
    fn new(lbound: isize, rbound: isize, rate_percent: isize, offset: isize) -> Self {
        RateUnit {
            lbound,
            rbound,
            rate_percent,
            offset,
        }
    }

    fn calc(&self, shotoku: isize) -> Option<isize> {
        if shotoku < self.rbound && shotoku >= self.lbound {
            Some(shotoku * self.rate_percent / 100 + self.offset)
        } else {
            None
        }
    }
}

struct RateScheme {
    ratelist: Vec<RateUnit>,
}

impl RateScheme {
    fn for_shotokuzei() -> Self {
        let ratelist = vec![
            RateUnit::new(0, 1_950_000, 5, 0),
            RateUnit::new(1_950_000, 3_300_000, 10, -97_500),
            RateUnit::new(3_300_000, 6_950_000, 20, -427_500),
            RateUnit::new(6_950_000, 9_000_000, 23, -636_000),
            RateUnit::new(9_000_000, 180_000_000, 33, -1_536_000),
            RateUnit::new(180_000_000, 40_000_000, 40, -2_796_000),
            RateUnit::new(40_000_000, isize::MAX, 45, -4_796_000),
        ];
        RateScheme { ratelist }
    }

    fn for_kisokojo() -> Self {
        let ratelist = vec![
            RateUnit::new(0, 24_000_001, 0, 480_000),
            RateUnit::new(24_000_001, 24_500_001, 0, 320_000),
            RateUnit::new(24_500_001, 25_000_001, 0, 160_000),
            RateUnit::new(25_000_001, isize::MAX, 0, 0),
        ];
        RateScheme { ratelist }
    }

    fn for_kyuyokojo() -> Self {
        let ratelist = vec![
            RateUnit::new(0, 1_625_001, 0, 550_000),
            RateUnit::new(1_625_001, 1_800_001, 40, -100_000),
            RateUnit::new(1_800_001, 3_600_001, 30, 80_000),
            RateUnit::new(3_600_001, 6_600_001, 20, 440_000),
            RateUnit::new(6_600_001, 8_500_001, 10, 1_100_000),
            RateUnit::new(8_500_001, isize::MAX, 0, 1_950_000),
        ];
        RateScheme { ratelist }
    }

    fn for_std_monthly_kenpo() -> Self {
        let ratelist = vec![
            RateUnit::new(0, 63_000, 0, 58_000),
            RateUnit::new(63_000, 73_000, 0, 68_000),
            RateUnit::new(73_000, 83_000, 0, 78_000),
            RateUnit::new(83_000, 93_000, 0, 88_000),
            RateUnit::new(93_000, 101_000, 0, 98_000),
            RateUnit::new(101_000, 107_000, 0, 104_000),
            RateUnit::new(107_000, 114_000, 0, 110_000),
            RateUnit::new(114_000, 122_000, 0, 118_000),
            RateUnit::new(122_000, 130_000, 0, 126_000),
            RateUnit::new(130_000, 138_000, 0, 134_000),
            RateUnit::new(138_000, 146_000, 0, 142_000),
            RateUnit::new(146_000, 155_000, 0, 150_000),
            RateUnit::new(155_000, 165_000, 0, 160_000),
            RateUnit::new(165_000, 175_000, 0, 170_000),
            RateUnit::new(175_000, 185_000, 0, 180_000),
            RateUnit::new(185_000, 195_000, 0, 190_000),
            RateUnit::new(195_000, 210_000, 0, 200_000),
            RateUnit::new(210_000, 230_000, 0, 220_000),
            RateUnit::new(230_000, 250_000, 0, 240_000),
            RateUnit::new(250_000, 270_000, 0, 260_000),
            RateUnit::new(270_000, 290_000, 0, 280_000),
            RateUnit::new(290_000, 310_000, 0, 300_000),
            RateUnit::new(310_000, 330_000, 0, 320_000),
            RateUnit::new(330_000, 350_000, 0, 340_000),
            RateUnit::new(350_000, 370_000, 0, 360_000),
            RateUnit::new(370_000, 395_000, 0, 380_000),
            RateUnit::new(395_000, 425_000, 0, 410_000),
            RateUnit::new(425_000, 455_000, 0, 440_000),
            RateUnit::new(455_000, 485_000, 0, 470_000),
            RateUnit::new(485_000, 515_000, 0, 500_000),
            RateUnit::new(515_000, 545_000, 0, 530_000),
            RateUnit::new(545_000, 575_000, 0, 560_000),
            RateUnit::new(575_000, 605_000, 0, 590_000),
            RateUnit::new(605_000, 635_000, 0, 620_000),
            RateUnit::new(635_000, 665_000, 0, 650_000),
            RateUnit::new(665_000, 695_000, 0, 680_000),
            RateUnit::new(695_000, 730_000, 0, 710_000),
            RateUnit::new(730_000, 770_000, 0, 750_000),
            RateUnit::new(770_000, 810_000, 0, 790_000),
            RateUnit::new(810_000, 855_000, 0, 830_000),
            RateUnit::new(855_000, 905_000, 0, 880_000),
            RateUnit::new(905_000, 955_000, 0, 930_000),
            RateUnit::new(955_000, 1_005_000, 0, 980_000),
            RateUnit::new(1_005_000, 1_055_000, 0, 1_030_000),
            RateUnit::new(1_055_000, 1_115_000, 0, 1_090_000),
            RateUnit::new(1_115_000, 1_175_000, 0, 1_150_000),
            RateUnit::new(1_175_000, 1_235_000, 0, 1_210_000),
            RateUnit::new(1_235_000, 1_295_000, 0, 1_270_000),
            RateUnit::new(1_295_000, 1_355_000, 0, 1_330_000),
            RateUnit::new(1_355_000, isize::MAX, 0, 1_390_000),
        ];
        RateScheme { ratelist }
    }

    fn for_std_monthly_nenkin() -> Self {
        let ratelist = vec![
            RateUnit::new(0, 93_000, 0, 88_000),
            RateUnit::new(93_000, 101_000, 0, 98_000),
            RateUnit::new(101_000, 107_000, 0, 104_000),
            RateUnit::new(107_000, 114_000, 0, 110_000),
            RateUnit::new(114_000, 122_000, 0, 118_000),
            RateUnit::new(122_000, 130_000, 0, 126_000),
            RateUnit::new(130_000, 138_000, 0, 134_000),
            RateUnit::new(138_000, 146_000, 0, 142_000),
            RateUnit::new(146_000, 155_000, 0, 150_000),
            RateUnit::new(155_000, 165_000, 0, 160_000),
            RateUnit::new(165_000, 175_000, 0, 170_000),
            RateUnit::new(175_000, 185_000, 0, 180_000),
            RateUnit::new(185_000, 195_000, 0, 190_000),
            RateUnit::new(195_000, 210_000, 0, 200_000),
            RateUnit::new(210_000, 230_000, 0, 220_000),
            RateUnit::new(230_000, 250_000, 0, 240_000),
            RateUnit::new(250_000, 270_000, 0, 260_000),
            RateUnit::new(270_000, 290_000, 0, 280_000),
            RateUnit::new(290_000, 310_000, 0, 300_000),
            RateUnit::new(310_000, 330_000, 0, 320_000),
            RateUnit::new(330_000, 350_000, 0, 340_000),
            RateUnit::new(350_000, 370_000, 0, 360_000),
            RateUnit::new(370_000, 395_000, 0, 380_000),
            RateUnit::new(395_000, 425_000, 0, 410_000),
            RateUnit::new(425_000, 455_000, 0, 440_000),
            RateUnit::new(455_000, 485_000, 0, 470_000),
            RateUnit::new(485_000, 515_000, 0, 500_000),
            RateUnit::new(515_000, 545_000, 0, 530_000),
            RateUnit::new(545_000, 575_000, 0, 560_000),
            RateUnit::new(575_000, 605_000, 0, 590_000),
            RateUnit::new(605_000, 635_000, 0, 620_000),
            RateUnit::new(635_000, isize::MAX, 0, 650_000),
        ];
        RateScheme { ratelist }
    }

    fn calc(&self, shotoku: isize) -> Option<isize> {
        for unit in self.ratelist.iter() {
            match unit.calc(shotoku) {
                Some(v) => {
                    return Some(v);
                }
                None => continue,
            }
        }
        None
    }
}

fn kirisute(val: f64) -> isize {
    (val - val % 1000.0) as isize
}

fn shakaihoken(nenshu: isize) -> isize {
    let std_monthly_kenpo = RateScheme::for_std_monthly_kenpo()
        .calc(kirisute(nenshu as f64 / 12.0))
        .unwrap();
    let std_monthly_nenkin = RateScheme::for_std_monthly_nenkin()
        .calc(kirisute(nenshu as f64 / 12.0))
        .unwrap();
    let kenpo = 0.5 * std_monthly_kenpo as f64 * KENKORATE;
    let nenkin = 0.5 * std_monthly_nenkin as f64 * NENKINRATE;
    12 * (kenpo + nenkin) as isize
}

fn nenshu_tedori_all(nenshu: isize) -> (isize, isize, isize, isize) {
    let kyuyokojo = RateScheme::for_kyuyokojo().calc(nenshu).unwrap();
    let gokeishotoku = nenshu - kyuyokojo;
    let kisokojo = RateScheme::for_kisokojo().calc(gokeishotoku).unwrap();
    let shaho = shakaihoken(nenshu);
    let kazeishotoku = match nenshu - shaho - kisokojo - kyuyokojo {
        v if v > 0 => v,
        _ => 0,
    };

    let zyuminzei = kazeishotoku / 10 + 5000;
    let shotokuzei = RateScheme::for_shotokuzei().calc(kazeishotoku).unwrap();
    let tedori = nenshu - shotokuzei - shaho - zyuminzei;
    (tedori, shaho, shotokuzei, zyuminzei)
}

fn nenshu_tedori(nenshu: isize) -> isize {
    let (tedori, _, _, _) = nenshu_tedori_all(nenshu);
    tedori
}

fn main() {
    println!("年収(円),所得税(円),社保(円),住民税(円),手取り(円),手取り率(%)");
    for nenshu in (1_000_000..20_000_001).step_by(100_000) {
        let (tedori, shaho, shotokuzei, zyuminzei) = nenshu_tedori_all(nenshu);
        println!(
            "{},{},{},{},{},{:.2}",
            nenshu,
            shotokuzei,
            shaho,
            zyuminzei,
            tedori,
            100.0 * tedori as f64 / nenshu as f64
        );
    }
}
