use std::{f32::consts::PI, thread, time};

fn main() {
    let mut output : [[char; screen_width as usize]; screen_width as usize] =  [[' '; screen_width as usize]; screen_width as usize];
    let mut a = 0.0;
    let mut b = 0.0;
    print!("\x1B[2J");
    loop {
        print!("\x1b[H");
        reder_frame(a, b, &mut output);
        
        for j in 0..screen_hight as usize{
            for i in 0..screen_width as usize {
                print!("{}", output[i][j]);
            }
            print!("\n");
        }
        output =  [[' '; screen_width as usize]; screen_width as usize];
        a += 0.2;
        b += 0.2;
        thread::sleep(time::Duration::from_millis(200));
    }
    
}

const theta_spacing : f32 = 0.07;
const phi_spacing : f32 = 0.02;

const R1 : f32 = 1.0;
const R2 : f32 = 2.0;
const K2 : f32 = 5.0;

const screen_width : f32 = 30.0;
const screen_hight : i32 = 30;

const K1 : f32 = screen_width * K2 * 3.0 / (8.0 * (R1 + R2));

//iterator for first loop
struct ThetaSpacingIter {
    curr: f32,
}
impl Iterator for ThetaSpacingIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.curr + theta_spacing;
        if self.curr >= 2.0 * PI {
            None
        } else {
            Some(current)
        }
    }
}

fn theta_spacing_seq() -> ThetaSpacingIter {
    ThetaSpacingIter { curr: 0.0 }
}

//iterator for second loop
struct PhiSpacingIter {
    curr: f32,
}
impl Iterator for PhiSpacingIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.curr + phi_spacing;
        if self.curr >= 2.0 * PI {
            None
        } else {
            Some(current)
        }
    }
}

fn phi_spacing_seq() -> PhiSpacingIter {
    PhiSpacingIter { curr: 0.0 }
}

fn reder_frame(a : f32, b: f32, output: &mut [[char; screen_width as usize]; screen_width as usize]) {
    let cos_a : f32 = f32::cos(a);
    let sin_a : f32 = f32::sin(a);
    let cos_b : f32 = f32::cos(b);
    let sin_b : f32 = f32::sin(b);

    let mut zbuffer : [[f32; screen_width as usize]; screen_width as usize] =  [[0.0; screen_width as usize]; screen_width as usize];

    let luminance_chars : Vec<char> = ".,-~:;=!*#$@".chars().collect();

    for theta in theta_spacing_seq() {
        let cos_theta : f32 = f32::cos(theta);
        let sin_theta : f32 = f32::sin(theta);

        for phi in phi_spacing_seq() {
            let cos_phi : f32 = f32::cos(phi);
            let sin_phi : f32 = f32::sin(phi);

            let circle_x : f32 = R2 + R1 * cos_theta;
            let circle_y : f32 = R1 * sin_theta;

            let x : f32 = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
            let y : f32 = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;
            let z : f32 = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz : f32 = 1.0 / z;

            let xp : usize = (screen_width / 2.0 + K1 * ooz * x) as usize; //TODO: tjek om typen skal være float eller i32
            let yp : usize = (screen_width / 2.0 - K1 * ooz * y) as usize;

            let L : f32 = cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            
            if L > 0.0 {
                if ooz > zbuffer[xp][yp] {
                    zbuffer[xp][yp] = ooz;
                    let luminance_index : usize = (L * 8.0) as usize;

                    output[xp][yp] = luminance_chars[luminance_index as usize]
                }
            }
        }
    }
}