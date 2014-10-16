use std::rand;
use std::num::FloatMath;
use serialize::json::{decode, Encoder};
use std::path::posix::Path;
use std::io::fs::File;
use std::io::{ReadWrite, Truncate, Open};
use  serialize::Encodable;
use std::io::Reader;

#[deriving(Encodable, Decodable, Clone, Show)]
struct Oscillator {
    pub phase: f64,
    pub omegadev: f64,
}

impl Oscillator {
    fn new() -> Oscillator {
        Oscillator {
            phase: rand::random::<f64>() * 360.0,
            omegadev: 0.99*(rand::random::<f64>()-0.5),
        }
    }
}
#[deriving(Encodable, Decodable, Clone, Show)]
struct Ret {
    zero: Vec<Oscillator>,
    all: Vec<Vec<f64>>
}

pub fn createSet(){
    let path = Path::new("data/30/new.dat");
    let mut file = File::open_mode(&path, Truncate, ReadWrite).unwrap();
    let mut encoder = Encoder::new(&mut file);
    let mut sum= 0.0;
    let mut arr = vec!();
    let mut i = 30u;
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
    arr.encode(&mut encoder);
}

pub fn run(n: uint, omega: f64, t: uint, K: f64, filename: String) -> (Vec<f64>) {
    let dt = 1f64;// 0.001; 
    let mut arr :Vec<Oscillator> = vec!();
    let mut ser = vec!();
    let mut sum = 0f64;
    let mut ret = vec!();
    let mut i = n;
    let K = K / n as f64;
    let path = Path::new(filename.as_slice());
    let mut file = File::open_mode(&path, Open, ReadWrite).unwrap();
    arr = decode(file.read_to_string().unwrap().as_slice()).unwrap();

    let arr0 = arr.clone();
    let mut ctr = 0;
    while ctr < t {
        let mut s = 0.0;
        let mut c = 0.0;
        let mut newarr = vec!();
        let mut buf = vec!();
        for i in arr.iter() {
            let mut sum = 0f64;
            for j in arr.iter() {
                sum += K*(((j.phase - i.phase)*3.1415/180.0).sin());
            }
            sum += i.omegadev*omega + omega;
            sum *= dt;
            sum += i.phase;
            sum -= 360f64*((sum/360.0) as uint as f64);
            let mut cl = i.clone();
            s+=(sum * 3.14159/180.0).sin();
            c+=(sum * 3.14159/180.0).cos();
            cl.phase = sum;
            buf.push(sum);
            newarr.push(cl)
        }
        println!("{} {}", ctr, (s*s + c*c).sqrt()/(n as f64) as f64)
        ret.push((s*s + c*c).sqrt()/(n as f64) as f64);
        arr = newarr;
        ser.push(buf);
        ctr += 1;
    }
//println!("{}", Encoder::str_encode(&Ret{zero: arr0.clone(), all: ser}))
//(arr0, arr)
ret
}

pub fn run_star(n: uint, omega: f64, t: uint, K: f64) -> (Vec<Oscillator>,Vec<Oscillator>) {
    let dt = 0.001;
    let mut arr = vec!();
    let mut sum = 0f64;
    let mainosc = Oscillator::new();
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
        let mut  sum = 0.0;
        for i in arr.mut_iter() {
            let dev =  K*(((i.phase - mainosc.phase)*3.1415/180.0).sin());
            sum += dev;
            i.phase += dt*(-dev + (i.omegadev + omega));
            i.phase -= 360f64*((i.phase/360.0) as uint as f64);
        }   
        sum += mainosc.omegadev*omega + omega;
        sum *= dt;
        sum += mainosc.phase;
        sum -= 360f64*((sum/360.0) as uint as f64);
        ctr -= 1;
    }
//println!("{}", Encoder::str_encode(&Ret{zero: arr0.clone(), all: ser}))
(arr0, arr)
}