use std::rand;
use std::num::FloatMath;
use serialize::json::Encoder;

#[deriving(Encodable, Decodable, Clone, Show)]
struct Oscillator {
    pub phase: f64,
    pub omegadev: f64,
}

impl Oscillator {
    fn new() -> Oscillator {
        Oscillator {
            phase: rand::random::<f64>() * 360.0,
            omegadev: 0.1*(rand::random::<f64>()-0.5),
        }
    }
}
#[deriving(Encodable, Decodable, Clone, Show)]
struct Ret {
    zero: Vec<Oscillator>,
    all: Vec<Vec<f64>>
}

pub fn run(n: uint, omega: f64, t: uint, K: f64) -> (Vec<Oscillator>,Vec<Oscillator>) {
    let dt = 0.001;
    let mut arr = vec!();
    let mut ser = vec!();
    let mut sum = 0f64;

    let mut i = n;
    let K = K / n as f64;
    while i > 0 {
        let osc = Oscillator::new();
        sum = sum + osc.phase;
        sum -= 360f64*((sum/360.0) as uint as f64);
        arr.push(osc);
        i -= 1;
    }
    for i in arr.mut_iter() {
        i.phase -= sum;
    }
    let arr0 = arr.clone();
    let mut ctr = t;
    while ctr > 0 {
        let mut newarr = vec!();
        let mut buf = vec!();
        for i in arr.iter() {
            let mut  sum = 0.0;
            for j in arr.iter() {
                sum += K*(((j.phase - i.phase)*3.1415/180.0).sin());
            }
            sum += i.omegadev*omega + omega;
            sum *= dt;
            sum += i.phase;
            sum -= 360f64*((sum/360.0) as uint as f64);
            let mut cl = i.clone();
            cl.phase = sum;
            buf.push(sum);
            newarr.push(cl)
        }
        arr = newarr;
        ser.push(buf);
        ctr -= 1;
    }
//println!("{}", Encoder::str_encode(&Ret{zero: arr0.clone(), all: ser}))
(arr0, arr)
}