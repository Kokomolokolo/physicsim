// Ein einfacher Physik Simulator mit Rust im wasm und auch mit Rocket Backend
// TODO: Alles schreib ich jetzt nicht alles hin

// import init, { update, get_positons} from "/static/physiksim.js"



function sim_loop () {
    update(); //ruft die rust funktionen auf, um updates über die physik zu bekommen

    const positions = get_positons(); // ruft die positonen auf die alle Kugeln haben, gibt nur daten zurück
                                      // Soll in Zukuft ein Array ausgeben so dass mit mehr Kugeln gearbeitet werden kann  

    draw(); // Malt neue Positionen

    requestAnimationFrame(sim_loop)
}

let ball

// Context für den Canvas
const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');
canvas.width = 1000;
canvas.height= 750;
// Malt
function draw() {

}
draw();
