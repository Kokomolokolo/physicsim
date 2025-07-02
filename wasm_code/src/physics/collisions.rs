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

// Impulsbasierte Kollsionslösung, ohne Reibung
pub fn handle_collisons_between_balls_v2(ball1: &mut Ball, ball2: &mut Ball, elasticity: f32) {
    // * KOLISIONSERKENNUNG * //
    // Distanz zwischen beiden Bällen Berechnen
    let dstx = ball2.x - ball1.x;
    let dsty = ball2.y - ball1.y;

    // Via Satz des Pythagoras yay. dx und dy sind die Katheten. Die Wurzel ihrer Summe ist der Abstand
    // der beiden Kugeln. Da eine sqr Rechnung aber teuer ist, Arbeite ich mit distance_squared 
    let dst_squared = dstx * dstx + dsty * dsty;
    let min_distance = ball1.radius + ball2.radius;
    let min_distance_squared = min_distance * min_distance;

    // Wenn Kollision:
    if dst_squared < min_distance_squared {
        // Die absotule Distance zwischen den Bällen
        let dst = dst_squared.sqrt();

        // Egde Case: Bälle sind aufeinader
        if dst == 0.0 {
            ball1.x -= 0.001;
            ball1.y -= 0.001;
        }

        // v_rel ist die relative Geschwindigkeit der beiden Bälle zueinander
        let v_rel = (
            ball1.dx - ball2.dx,
            ball1.dy - ball2.dy 
        );
        
        // Der Kollisionsnormalenvektor. Gibt an welchen Vektor die Kollsion ist -> Richtung der Kolision
        // Der Vektor ist normalisiert. DAS BITTE LERNEN. also immer eins da er durch dst geteilt ist
        let normal_vec = (
            dstx / dst,
            dsty / dst
        );
        // Die normalisierte relative Geschwindigkeit.
        // Errechnen eines Skalerproduktes das sagt wie doll die beiden Bälle aufeinader stoßen.
        // relative v entlang des normalvektors
        let v_rel_norm = (v_rel.0 * normal_vec.0) + (v_rel.1 * normal_vec.1); // goofy ahh index bei tupeln

        // Inverse Massen: Wie stark werden die Bälle beeinflusst
        let inv_mass_ball1 = 1.0 / ball1.mass;
        let inv_mass_ball2 = 1.0 / ball2.mass;
        let inv_mass_total = inv_mass_ball1 + inv_mass_ball2;
        
        
        
        // Wenn sie aufeinander zu gehen
        if v_rel_norm < 0.0 {
            // Impulskoeffizienten-Formel. Nimmt die elastizität + 1 und die relative geschwindigkeit durch die torale inv masse.
            let imp_koeffi = - ( (1.0 + elasticity) * v_rel_norm ) / inv_mass_total;

            // * Geschwindigkeiten einpassen * //
            // der impuls wird auf die normalisierten vektoren angewendet.
            // Also die stärke des Aufpralles * die richtung des aufpralles * masse sind der neute geschwindigkeitsverktor
            ball1.dx += imp_koeffi * normal_vec.0 * inv_mass_ball1;
            ball1.dy += imp_koeffi * normal_vec.1 * inv_mass_ball1;

            //ball2 - da andere Richtung
            ball2.dx -= imp_koeffi * normal_vec.0 * inv_mass_ball2;
            ball2.dy -= imp_koeffi * normal_vec.1 * inv_mass_ball2;
        }

        // * Penetration Korrektur * //
        let penetration_depth = min_distance - dst;

        // adjuted_pen_depth aufgrund von kleinen Rechnefehlern mit floating
        // Korrektur für "slack" wie es der random auf stack overflow nennt
        let slack_tolerance = 0.0;
        let adjusted_pen_depth = (penetration_depth - slack_tolerance).max(0.0);

        // Prozent Wert welcher der Bälle wie stark bewegt wird abhänig von der Masse.
        let percent_ball1 = inv_mass_ball1 / inv_mass_total;
        let percent_ball2 = inv_mass_ball2 / inv_mass_total;

        // Ball 1 verschiebt sich ENTGEGEN der Richtung des normal_vec
        ball1.x -= adjusted_pen_depth * percent_ball1 * normal_vec.0;
        ball1.y -= adjusted_pen_depth * percent_ball1 * normal_vec.1;

        // Ball 2 verschiebt sich ENTLANG der Richtung des normal_vec
        ball2.x += adjusted_pen_depth * percent_ball2 * normal_vec.0; // <-- Korrektur! Vorzeichen
        ball2.y += adjusted_pen_depth * percent_ball2 * normal_vec.1; // <-- Korrektur! Vorzeichen

    }
}