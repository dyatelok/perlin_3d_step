use raylib::prelude::*;
use rand::{thread_rng, Rng};
use euler::*;

//Генерируем 3д шум Перлина в кубе. Подавать значения координат от 0 до 1
fn perlin(x : f32, y : f32, z : f32, v : &Vec<Vec<Vec<Vec3>>>) -> f32 {
    let X = x * (v.len()-1) as f32;
    let Y = y * (v.len()-1) as f32;
    let Z = z * (v.len()-1) as f32;

    let i : usize = X.floor() as usize;
    let j : usize = Y.floor() as usize;
    let l : usize = Z.floor() as usize;
    
    let I = i as f32;
    let J = j as f32;
    let L = l as f32;

    let xx = X - i as f32;
    let yy = Y - j as f32;
    let zz = Z - l as f32;

    let mut vv : Vec3;
    let mut G : Vec<Vec<Vec<f32>>> = vec!(vec!(vec!(0.0; 2); 2); 2);

    vv = vec3!(X-I    ,Y-J    ,Z-L    );//.normalize();
    G[0][0][0] = vv.dot(v[i  ][j  ][l  ]);

    vv = vec3!(X-I-1.0,Y-J    ,Z-L    );//.normalize();
    G[1][0][0] = vv.dot(v[i+1][j  ][l  ]);

    vv = vec3!(X-I    ,Y-J-1.0,Z-L    );//.normalize();
    G[0][1][0] = vv.dot(v[i  ][j+1][l  ]);
    
    vv = vec3!(X-I-1.0,Y-J-1.0,Z-L    );//.normalize();
    G[1][1][0] = vv.dot(v[i+1][j+1][l  ]);

    vv = vec3!(X-I    ,Y-J    ,Z-L-1.0);//.normalize();
    G[0][0][1] = vv.dot(v[i  ][j  ][l+1]);

    vv = vec3!(X-I-1.0,Y-J    ,Z-L-1.0);//.normalize();
    G[1][0][1] = vv.dot(v[i+1][j  ][l+1]);

    vv = vec3!(X-I    ,Y-J-1.0,Z-L-1.0);//.normalize();
    G[0][1][1] = vv.dot(v[i  ][j+1][l+1]);
    
    vv = vec3!(X-I-1.0,Y-J-1.0,Z-L-1.0);//.normalize();
    G[1][1][1] = vv.dot(v[i+1][j+1][l+1]);

    interp(G,xx,yy,zz)
}

fn interp(G : Vec<Vec<Vec<f32>>>, x : f32, y : f32, z : f32) -> f32 {
    let xx = fade(x);
    let yy = fade(y);
    let zz = fade(z);
    let a0 = lerp(G[0][0][0],G[1][0][0],xx);
    let b0 = lerp(G[0][1][0],G[1][1][0],xx);
    let c0 = lerp(a0,b0,yy);
    let a1 = lerp(G[0][0][1],G[1][0][1],xx);
    let b1 = lerp(G[0][1][1],G[1][1][1],xx);
    let c1 = lerp(a1,b1,yy);
    lerp(c0,c1,zz)
}

fn fade(t : f32) -> f32{
	t * t * t * (t * (t * 6.0 - 15.0) + 10.0)			// 6t^5 - 15t^4 + 10t^3
}

fn lerp(a : f32, b : f32, x : f32) -> f32 {
    a + (b - a) * x
}

fn CC(pot : f32) -> Color {
    Color::new(pot as u8, pot as u8, pot as u8, 255)
}

//Генерируем 3д шум Перлина в кубе. Подавать значения координат от 0 до 1
fn perlin_color(x : f32, y : f32, z : f32, v : &Vec<Vec<Vec<Vec3>>>) -> Color {
    CC( ((((perlin(x,y,z,v) + 1.0) / 2.0 * 255.0) .floor() * 12.0) as i32 % 256) as f32)
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 600)
        .title("perlin_3d_step")
        .build();
    let mut v : Vec<Vec<Vec<Vec3>>> = Vec::new();
    let r : usize = 10+1;
    let mut rng = rand::thread_rng();
    v.resize(r,Vec::new());
    for i in 0..r {
        v[i].resize(r,Vec::new());
        for j in 0..r {
            v[i][j].resize(r,vec3!());
            for l in 0..r {
            let mut x : f32 = rng.gen(); x = x - 0.5;
            let mut y : f32 = rng.gen(); y = y - 0.5;
            let mut z : f32 = rng.gen(); z = z - 0.5;
            v[i][j][l] = vec3!(x,y,z).normalize();
            }
        }
    }

    let mut l : i32 = 0;
    rl.set_target_fps(100);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        /*for i in 0..599 {
            for j in 0..599 {
                d.draw_pixel(i, j, perlin_color(i as f32 / 600.0 , j as f32 / 600.0, l as f32 / 600.0, &v));
            }
        }*/
        for i in 0..300 {
            for j in 0..300 {
                d.draw_rectangle(i * 2, j * 2, 2, 2, perlin_color(i as f32 / 600.0 , j as f32 / 600.0, l as f32 / 600.0, &v));
            }
        }
        l += 2;
    }
}
