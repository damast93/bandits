
pub fn argmax(v : &Vec<f32>) -> usize {
    let mut i = 0;
    let mut ymax = -1.0;
    for (j,y) in v.iter().enumerate() {
        if *y > ymax {
            ymax = *y;
            i = j;
        }
    }
    i
}
