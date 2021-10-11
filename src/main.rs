mod rk4;

fn main() {
    use rk4::RK4;

    let f = |x, y| { -2.0*x*y };
    let mut p: RK4<f32> = RK4 { x: 0.0, y: 1.0, step: 0.1, f };
    
    for i in 1..=20 {
        println!("{}, {:?}", i, p.next().unwrap());
    }
}
