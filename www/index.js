import { GameData } from "wasm-game-of-life";

let num_player = prompt("プレイヤーの人数を選択してください(1〜4まで選択できます)");

let gamedata = GameData.new();

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
    case " ":
    gamedata.put_bomb(b, 0);
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
    case "x":
    gamedata.put_bomb(b, 1);
    break;

    case "f":
    gamedata.toggle_left(b, 2);
    break;
    case "h":
    gamedata.toggle_right(b, 2);
    break;
    case "t":
    gamedata.toggle_up(b, 2);
    break;
    case "g":
    gamedata.toggle_down(b, 2);
    break;

    case "j":
    gamedata.toggle_left(b, 3);
    break;
    case "l":
    gamedata.toggle_right(b, 3);
    break;
    case "i":
    gamedata.toggle_up(b, 3);
    break;
    case "k":
    gamedata.toggle_down(b, 3);
    break;
    } 
}


document.addEventListener('keydown', e => processKey(e.key, true));
document.addEventListener('keyup', e => processKey(e.key, false));
/*
function resize() {
    gamedata = GameData.new();
}
// Resizing
window.addEventListener('resize', () => {
    resize();
    let i = 0;
    let player_x = 300;
    let player_y = 200;
    for(i = 0; i < num_player; i = i + 1){
        gamedata.create_player(player_x, player_y);
        player_x = player_x + 100;
        player_y = player_y + 100;
    }    
});
*/

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
/*
let i = 0;
let player_x = 300;
let player_y = 200;
for(i = 0; i < num_player; i = i + 1){
    gamedata.create_player(player_x, player_y);
    player_x = player_x + 100;
    player_y = player_y + 100;
}
*/
gamedata.create_player(175,175);
gamedata.create_player(575,525);

gamedata.create_wall();

drawAndUpdate();
//});
