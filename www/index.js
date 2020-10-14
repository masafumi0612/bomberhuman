import { GameData } from "wasm-game-of-life";

let gamedata = GameData.new(512.0, 300.0)

// Input processing
function processKey(key, b) {
    switch (key) {
    case "ArrowLeft":
	gamedata.toggle_left(b, 0);
	break;
    case "ArrowRight":
	gamedata.toggle_right(b, 0);
	break;
    case "ArrowUp":
	gamedata.toggle_up(b, 0);
    break;
    case "ArrowDown":
	gamedata.toggle_down(b, 0);
    break;

    case "a":
    gamedata.toggle_left(b, 1);
    break;
    case "d":
    gamedata.toggle_right(b, 1);
    break;
    case "w":
    gamedata.toggle_up(b, 1);
    break;
    case "s":
    gamedata.toggle_down(b, 1);
    break;
    }
}


document.addEventListener('keydown', e => processKey(e.key, true));
document.addEventListener('keyup', e => processKey(e.key, false));

function resize() {
    gamedata = GameData.new();
}
// Resizing
window.addEventListener('resize', () => {
    resize();
    gamedata.create_player(300, 200);
    gamedata.create_player(800, 600);
});


// Game loop
let start = null;
let prevTimestamp = null;
let drawAndUpdate = (timestamp) => {
    // Initialization
    if (!prevTimestamp) {
	start = timestamp;
	prevTimestamp = timestamp;
	requestAnimationFrame(drawAndUpdate);
	return;
    }

    // Update and draw
    let progress = (timestamp - prevTimestamp) / 1000;
    gamedata.update(progress);
//    clear_screen();
    gamedata.draw();

    // Some bookkeeping
    prevTimestamp = timestamp;
    requestAnimationFrame(drawAndUpdate);
};

gamedata.create_player(300, 200);
gamedata.create_player(800, 600);
drawAndUpdate();
//});
