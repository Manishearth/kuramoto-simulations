use std::rand;
use serialize::json::{decode, encode};
use std::path::posix::Path;
use std::old_io::fs::File;
use std::old_io::{ReadWrite, Truncate, Open};
use  serialize::Encodable;
use std::old_io::Reader;
use std::num::Float;

#[derive(Encodable, Decodable, Clone, Show)]
struct Oscillator {
    pub phase: f64,
    pub omegadev: f64,
}

impl Oscillator {
    fn new() -> Oscillator {
        Oscillator {
            phase: rand::random::<f64>() * 360.0,
            omegadev: 0.01*(rand::random::<f64>()-0.5),
        }
    }
}
#[derive(Encodable, Decodable, Clone, Show)]
struct Ret {
    zero: Vec<Oscillator>,
    all: Vec<Vec<f64>>
}

pub fn createSet(){
    let path = Path::new("data/30/new.dat");
    let mut file = File::open_mode(&path, Truncate, ReadWrite).unwrap();
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
    for i in arr.iter_mut() {
        i.phase -= sum;
    }
    let s = encode(&arr);
    file.write_str(s.unwrap().as_slice());
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
        //println!("{} {}", ctr, (s*s + c*c).sqrt()/(n as f64) as f64)
        ret.push((s*s + c*c).sqrt()/(n as f64) as f64);
        arr = newarr;
        ser.push(buf);
        ctr += 1;
    }
println!("{:?}", encode(&Ret{zero: arr0.clone(), all: ser}));
//(arr0, arr)
ret
}

pub fn run_star(n: uint, omega: f64, t: uint, K: f64, filename: String) -> (Vec<f64>) {
    let dt = 1f64;// 0.001; 
    let mut sum = 0f64;
    let mut ret = vec!();
    let mut ser = vec!();
    let mut i = n;
    let K = K / n as f64;
    let path = Path::new(filename.as_slice());
    let mut file = File::open_mode(&path, Open, ReadWrite).unwrap();
    let arr_unsplit :Vec<Oscillator> = decode(file.read_to_string().unwrap().as_slice()).unwrap();
    let mut arr = arr_unsplit.tail().to_vec();
    let arr0 = arr.clone();
    let mut main = arr_unsplit[0].clone();
    let mut ctr = 0;
    while ctr < t {
        let mut s = 0.0;
        let mut c = 0.0;
        let mut newarr = vec!();
        let mut buf = vec!();
        let mut sum = main.omegadev*omega*n as f64 + omega;
        for i in arr.iter() {
            let newphase =  dt*(K*(((main.phase -i.phase)*3.1415/180.0).sin())+(i.omegadev*omega) + omega) + i.phase;
            let mut j = i.clone();
            j.phase = newphase;
            //println!("new {}", newphase)
            j.phase -= 360f64*((newphase/360.0) as int as f64);
            newarr.push(j);
            buf.push(newphase);
            sum += K*(((i.phase - main.phase)*3.1415/180.0).sin());
            s+=(newphase * 3.14159/180.0).sin();
            c+=(newphase * 3.14159/180.0).cos();

        }
        sum *= dt;
        sum += main.phase;
        //println!("presum, {}", sum)
        sum -= 360f64*((sum/360.0) as int as f64);
        //println!("sum, {}", sum)

        //println!("s{},c{}",s,c)

            main.phase = sum;
        //s+=(main.phase * 3.14159/180.0).sin();
        //c+=(main.phase * 3.14159/180.0).cos();
        //println!("{} {}", ctr, (s*s + c*c).sqrt()/(n as f64) as f64)
        ret.push((s*s + c*c).sqrt()/(n as f64 - 1f64) as f64);
        ser.push(buf);
        arr = newarr;
        ctr += 1;
    }
println!("{:?}", encode(&Ret{zero: arr0.clone(), all: ser}));
//(arr0, arr)
ret
}

pub fn run_star_stateful(init: Option<(Vec<Oscillator>, Oscillator)>, n: uint, omega: f64, t: uint, K: f64, filename: String) -> (Vec<f64>, (Vec<Oscillator>, Oscillator)) {
    let dt = 1f64;// 0.001; 
    let mut sum = 0f64;
    let mut ret = vec!();
    let mut ser = vec!();
    let mut i = n;
    let K = K / n as f64;
    let (arr0, mut main) = match init {
        Some(i) => i,
        None => {
            let path = Path::new(filename.as_slice());
            let mut file = File::open_mode(&path, Open, ReadWrite).unwrap();
            let arr_unsplit :Vec<Oscillator> = decode(file.read_to_string().unwrap().as_slice()).unwrap();
            let mut arr = arr_unsplit.tail().to_vec();
            (arr.clone(), arr_unsplit[0].clone())
        }
    };
    let mut arr = arr0.clone();
    let mut ctr = 0;
    while ctr < t {
        let mut s = 0.0;
        let mut c = 0.0;
        let mut newarr = vec!();
        let mut buf = vec!();
        let mut sum = main.omegadev*omega*n as f64 + omega;
        for i in arr.iter() {
            let newphase =  dt*(K*(((main.phase -i.phase)*3.1415/180.0).sin())+(i.omegadev*omega) + omega) + i.phase;
            let mut j = i.clone();
            j.phase = newphase;
            //println!("new {}", newphase)
            j.phase -= 360f64*((newphase/360.0) as int as f64);
            newarr.push(j);
            buf.push(newphase);
            sum += K*(((i.phase - main.phase)*3.1415/180.0).sin());
            s+=(newphase * 3.14159/180.0).sin();
            c+=(newphase * 3.14159/180.0).cos();

        }
        sum *= dt;
        sum += main.phase;
        //println!("presum, {}", sum)
        sum -= 360f64*((sum/360.0) as int as f64);
        //println!("sum, {}", sum)

        //println!("s{},c{}",s,c)

            main.phase = sum;
        //s+=(main.phase * 3.14159/180.0).sin();
        //c+=(main.phase * 3.14159/180.0).cos();
        //println!("{} {}", ctr, (s*s + c*c).sqrt()/(n as f64) as f64)
        ret.push((s*s + c*c).sqrt()/(n as f64 - 1f64) as f64);
        ser.push(buf);
        arr = newarr;
        ctr += 1;
    }
//println!("{}", encode(&Ret{zero: arr0.clone(), all: ser}));
//(arr0, arr)
(ret, (arr, main))
}