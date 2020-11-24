import { GameData } from "wasm-game-of-life";

//let num_player = prompt("プレイヤーの人数を選択してください(1〜4まで選択できます)");

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
    case "b":
    gamedata.put_bomb(b, 2);
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
    case "b":
    gamedata.put_bomb(b, 3);
    break;
    } 
}

////////////////////////////////////////////////////////////////
// Gamepads handling
////////////////////////////////////////////////////////////////

// Needs "gamepadconnected" handler even if empty.
function init_gamepads(gp) {
    if (debug) {
      console.log("Gamepad connected at index:%d buttons:%d axes:%d [%s]",
                  gp.index, gp.buttons.length, gp.axes.length, gp.id);
    }
  }
  
  function scan_gamepads() {
    // Chrome should refresh gamepads everytime you read.
    var gamepads = navigator.getGamepads ? navigator.getGamepads() : [];
  
    for (var i = 0; i < gamepads.length; i++) {
      var pad = gamepads[i];
      if (pad) {
        // Send state to WASM
        if (pad.axes[0] < -0.5) {
            gamedata.toggle_left(true, i);
        } else {
            gamedata.toggle_left(false, i);
        }
        if (pad.axes[0] >  0.5) {
            gamedata.toggle_right(true, i);
        } else {
            gamedata.toggle_right(false, i);
        }
        if (pad.axes[1] <  -0.5) {
            gamedata.toggle_up(true, i);
        }else {
            gamedata.toggle_up(false, i);
        }
        if (pad.axes[1] >  0.5) {
            gamedata.toggle_down(true, i);
        }else {
            gamedata.toggle_down(false, i);
        }
        if (pad.buttons[0].pressed) {
            gamedata.put_bomb(true, i);
        } else {
            gamedata.put_bomb(false, i);
        }
      }
    }
  }  

document.addEventListener('keydown', e => processKey(e.key, true));
document.addEventListener('keyup', e => processKey(e.key, false));
document.addEventListener("gamepadconnected", e => init_gamepads(e.gamepad));

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
    scan_gamepads();
    gamedata.update(progress);
//    clear_screen();
    gamedata.draw();

    // Some bookkeeping
    prevTimestamp = timestamp;
    requestAnimationFrame(drawAndUpdate);
};
gamedata.create_player(75,75);
gamedata.create_player(675,75);
gamedata.create_player(75,575);
gamedata.create_player(675,575);

gamedata.create_wall();
gamedata.create_sblock();
gamedata.create_pow();

drawAndUpdate();
//});
