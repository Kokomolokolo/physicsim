use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

// Allg Aufbau: BallManager hat ein Vector, worin alle Bälle mit id, x, y, dx, dy, radius (obwohl der unnötig ist)
// sind. Js ruft eine rust funktion update_and_get_positions auf. Diese itteriert durch
// den Vector via for ball in &mut self balls. Errechnet für jeden Ball die neuen positionen in den Vector.
// Die Methode gibt bestimme Werte wie id, x, y, radius, zurück und so in js gemalt.
// Es wird ein flaches Array verwendet also wird in js for (let x = 0, x <= arr.len, x = x + 4)

// Methode add_ball schreibt einen neuen Ball type in Balls vie balls.push(Ball)

// Dann wird für dieses gemalt. Wow warum ist das so ein großes Projekt??



// Intitilasiert einen Ball, später vielleicht auch noch rgb werte?
// Kann nicht einfach initialisiert werden,
// da es nicht global bzw. mut geht ohne data races.
// Das ist so hoffentlich mehr oop. yay?
#[wasm_bindgen]
pub struct Ball {
    pub id: u32, // Die ID wird zurück gegeben
    pub dx: f32,
    pub dy: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32, // Sind alle in f32 da f32 + i32 = AAAAAHHH
} 
#[wasm_bindgen]
pub struct BallManager {
    #[wasm_bindgen(skip)]
    balls: Vec<Ball>, // Vector mit allen Bällen
    next_id: u32, 
    canvas_width: f32, // siehe radius
    canvas_height: f32, // werden später bei kollision verwendet
}

// Implementierung des BallManagers
#[wasm_bindgen]
impl BallManager {
    // Erstellt neuen Ballmanager, wird anfangs einmalig im js gecalled
    pub fn new(canvas_width: f32, canvas_height: f32) -> Self { // Nimmt width und height, gibt Ballmanager zurück
        BallManager {
            balls: Vec::new(), // Leeren Vector Balls
            next_id: 0,
            canvas_width,
            canvas_height,
        }
    }

    // Methode fügt Bälle hinzu, wow
    pub fn add_ball(&mut self, x: f32, y: f32, dx: f32, dy: f32, radius: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1; // Aktualisiert die nächte verfügbare ID in Ballmanager
        self.balls.push( Ball {
            x,
            y,
            dx,
            dy,
            radius,
            id,
        });
        id
    }
    // Berechnet alle bewegungen und schreibt sie in Ballmanager.balls,
    // gibt einen Vector aus. Muss ein Float32Array sein, da wasm_bindgen
    // keine komplexen Datatypen wie ein vec verweden kann. So geht das hoffentlich (foreshadowing?)
    pub fn update_and_get_positions(&mut self) -> Float32Array {
        let mut data = Vec::with_capacity(self.balls.len() * 4);  //Vec mit platz für id, x, y, radius für jeden ball

        for ball in &mut self.balls {
            // Bewegung
            ball.x = ball.x + ball.dx;
            ball.y = ball.y + ball.dy;
           
            // *** Kollisionslogik ***
            // Kolision mit oben und unten
            if ball.y + ball.radius >= self.canvas_height || ball.y - ball.radius <= 0.0 {
                ball.dy *= -1.0;
            }
            // Kolision mit Wänden
            if ball.x + ball.radius >= self.canvas_width || ball.x -ball.radius <= 0.0 {
                ball.dx *= -1.0;
            }
            // Korigierte Positonen bei Feststecken
            if ball.x + ball.radius >= self.canvas_width {
                ball.x = self.canvas_width - ball.radius; // Position korrigieren
                ball.dx *= -1.0;
            } else if ball.x - ball.radius <= 0.0 {
                ball.x = ball.radius; // Position korrigieren
                ball.dx *= -1.0;
            }
        }

        // Kolison der Bälle untereinader
        // Nested For loops, keine dopelten checks
        // i ist immer kleiner als j, j iterriert über alle größeren als i, da die kleineren bereits
        // mit allem verglichen wurden
        for i in 0..self.balls.len(){ // range von null bis ball.len()
            for j in (i + 1)..self.balls.len() {
                let (ball1, ball2) = {
                    let (left_part, right_part) = self.balls.split_at_mut(j);
                    // Das teilt das Array an einem index in 2 Hälften. Das ist wohl wegen des Borror checkers wichtig.
                    (&mut left_part[i], &mut right_part[0])
                    // Gibt die mut Referencen zu einem Ball im linken Part und dem ersten im Rechten zurück
                };
                handle_colisons_between_balls_v1(ball1, ball2);
            }
        }
        for ball in &self.balls {
            //Füllen des Arrays
            data.push(ball.id as f32); // Umwandlung, da es ein f32 Array ist
            data.push(ball.x);
            data.push(ball.y);
            data.push(ball.radius);
        }
        Float32Array::from(&data[..]) // IDK WTF aber Vec<f32> wird zu Float32Array
    }

    fn handle_colisons_between_balls_v1(ball1: &mut Ball, ball2: &mut Ball) {
        // Distanz zwischen beiden Bällen Berechnen
                let dx = ball2.x - ball1.x;
                let dy = ball2.y - ball1.y;

                // Via Satz des Pythagoras yay. dx und dy sind die Katheten. Die Wurzel ihrer Summe ist der Abstand
                // der beiden Kugeln. Da eine sqr Rechnung aber teuer ist, Arbeite ich mit distance_squared 
                let distance_squared = dx * dx + dy * dy;
                let min_distance = ball1.radius + ball2.radius;
                let min_distance_squared = min_distance * min_distance;
                
                if distance_squared < min_distance_squared {
                    // *** Kolison *** 
                    ball1.dx *= -1.0;
                    ball1.dy *= -1.0;
                    ball2.dx *= -1.0;
                    ball2.dy *= -1.0;
                }
                // Sonderfälle um Überlappungen zu vermeiden
                if distance_squared == 0.0 { // Sonderfall,vermeidet bei uns eigentlich nur Division durch 0
                    ball1.x -= 0.1;
                    ball2.x += 0.1;
                } else {
                    let current_distance = distance_squared.sqrt();
                    let overlap = min_distance - current_distance;

                    if overlap > 0.0 {
                        // Normalisierung der Vektoren, ist logisch wenn ma drüber nachdenkt
                        let nx = dx / current_distance;
                        let ny = dy / current_distance;

                        // Bewegung je der hälfe des Overlaps gegeneiander
                        ball1.x -= overlap * 0.5 * nx;
                        ball1.y -= overlap * 0.5 * ny;
                        ball2.x += overlap * 0.5 * nx;
                        ball2.y += overlap * 0.5 * ny;
                    }
                }
    }
}
