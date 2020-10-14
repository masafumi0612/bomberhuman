// We create this here because it will be used from within `imports`
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext("2d");


// Returns an object containing resources that will be used later for drawing
function resources() {
    let res = {
	player: document.createElement('canvas'),
//	enemy: document.createElement('canvas'),
//	bullet: document.createElement('canvas'),
	particle: document.createElement('canvas')
    }

    // Player
    res.player.width = 20;
    res.player.height = 20;
    let plCtx = res.player.getContext('2d');
    plCtx.fillStyle = "red";
    plCtx.beginPath();
    plCtx.arc(10, 10, 10, 0, 2 * Math.PI);

    plCtx.fill();

    return res;
}

// Returns an object containing functions that will be linked to our wasm model
// This means that they can be called from Rust
const res = resources();

export class Draw {
    width() {
    canvas.width = window.innerWidth * 0.8;
	return canvas.width;
    }

    height() {
    canvas.height = window.innerHeight * 0.8;
	return canvas.height;
    }
    
    clear_screen() {
	ctx.fillStyle = "black";
	ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    draw_player(x, y, angle) {
	ctx.translate(x, y);
	ctx.rotate(angle);
	ctx.translate(0, -8);
	ctx.drawImage(res.player, 0, 0);
	ctx.setTransform(1, 0, 0, 1, 0, 0);
	
	ctx.fillStyle = "black";
    }

    draw_score(x) {
	ctx.fillStyle = "orange";
	ctx.textBaseline = "top";
	ctx.font = "20px sans-serif";
	ctx.fillText('Score: ' + x, 10, 10)
    }

    // The real loading and running of our wasm starts here
    //let imports = { clear_screen, draw_player, draw_enemy, draw_bullet, draw_particle, draw_score };
    //imports.Math_atan = Math.atan;
    //imports.sin = Math.sin;
    //imports.cos = Math.cos;
    
}
