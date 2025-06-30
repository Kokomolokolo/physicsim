use crate::ball::Ball;
// steht für smooth partical hydrodynamics
pub fn compute_sph_forces() {
    
}


// Gibt eine Zahl an wie doll ein Objekt auf ein anderes einen Einfluss hat.
// Je näher das Objekt, dest stärker ist der Einfluss.
fn smoothing_kernel(radius: f32, dist: f32) -> f32 {
    let value = (radius * radius - dist * dist).max(0.0);
    value * value * value
}

fn calculate_densicy(point: Vec<f32>) -> f32 {
    
}