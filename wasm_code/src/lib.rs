

// Allg Aufbau: BallManager hat ein Vector, worin alle Bälle mit id, x, y, dx, dy, radius (obwohl der unnötig ist)
// sind. Js ruft eine rust funktion update_and_get_positions auf. Diese itteriert durch
// den Vector via for ball in &mut self balls. Errechnet für jeden Ball die neuen positionen in den Vector.
// Die Methode gibt bestimme Werte wie id, x, y, radius, zurück und so in js gemalt.
// Es wird ein flaches Array verwendet also wird in js for (let x = 0, x <= arr.len, x = x + 4)

// Methode add_ball schreibt einen neuen Ball type in Balls vie balls.push(Ball)

// Dann wird für dieses gemalt. Wow warum ist das so ein großes Projekt??

mod ball;
mod ball_manager;
mod physics {
    pub mod collisions;
    pub mod sph;
}

pub use ball::Ball;
pub use ball_manager::BallManager;