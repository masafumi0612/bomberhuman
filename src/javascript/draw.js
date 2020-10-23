// We create this here because it will be used from within `imports`
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext("2d");


// Returns an object containing resources that will be used later for drawing
function resources() {
    let res = {
        player: document.createElement('canvas'),
        //	enemy: document.createElement('canvas'),
        //	bullet: document.createElement('canvas'),
        particle: document.createElement('canvas'),
        wall: document.createElement('canvas'),
    }

    // Player
    res.player.width = 50;
    res.player.height = 50;
    let plCtx = res.player.getContext('2d');
    plCtx.fillStyle = "red";
    plCtx.beginPath();
    plCtx.arc(25, 25, 25, 0, 2 * Math.PI);

    plCtx.fill();

    // Wall
    res.wall.width = 50;
    res.wall.height = 50;
    let wallCtx = res.wall.getContext('2d');
    wallCtx.fillStyle = "blue";
    wallCtx.beginPath();
    wallCtx.fillRect(0, 0, res.wall.width, res.wall.height);
    
    return res;
}

// Returns an object containing functions that will be linked to our wasm model
// This means that they can be called from Rust
const res = resources();

export class Draw {
    width(x) {
        //        canvas.width = window.innerWidth * 0.8;
        canvas.width = x;
        console.log(canvas.width);
        return canvas.width;
    }

    height(y) {
        //        canvas.height = window.innerHeight * 0.8;
        canvas.height = y;
        console.log(canvas.height);
        return canvas.height;
    }

    clear_screen() {
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    draw_player(x, y, angle) {
        ctx.translate(x-25, y-25);
        ctx.drawImage(res.player, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";  
    }

    draw_wall(x, y) {
        ctx.translate(x-25, y-25);
        ctx.drawImage(res.wall, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";  
    }
    

    // The real loading and running of our wasm starts here
    //let imports = { clear_screen, draw_player, draw_enemy, draw_bullet, draw_particle, draw_score };
    //imports.Math_atan = Math.atan;
    //imports.sin = Math.sin;
    //imports.cos = Math.cos;
    
}
