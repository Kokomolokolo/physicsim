use crate::ball::Ball;

pub fn handle_collisons_between_balls_v1(ball1: &mut Ball, ball2: &mut Ball, elasticity: f32) {
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
            ball1.dx *= -elasticity;
            ball1.dy *= -elasticity;
            ball2.dx *= -elasticity;
            ball2.dy *= -elasticity;
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