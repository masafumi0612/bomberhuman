// We create this here because it will be used from within `imports`
const canvas = document.getElementById('canvas');
let ctx = canvas.getContext("2d");


// Returns an object containing resources that will be used later for drawing
function resources() {
    let res = {
        player: document.createElement('img'),
        bomb_power: document.createElement('img'),
        bomb_num: document.createElement('img'),
        player_speed: document.createElement('img'),
        sblock: document.createElement('img'),
        wall: document.createElement('img'),
        bomb: document.createElement('img'),
        fire: document.createElement('img'),
    }

    // Player
    res.player.width = 50;
    res.player.height = 50;
    res.player.src = "image/player.png";

//    res.player.width = 50;
//    res.player.height = 50;

//    let plCtx = res.player.getContext('2d');
//    plCtx.fillStyle = "red";
//    plCtx.beginPath();
//    plCtx.arc(25, 25, 25, 0, 2 * Math.PI);
//    plCtx.fill();

    // Pow_bomb_power
    res.bomb_power.width = 50;
    res.bomb_power.height = 50;
    res.bomb_power.src = "image/bomb_power.png";

    // Pow_bomb_num
    res.bomb_num.width = 50;
    res.bomb_num.height = 50;
    res.bomb_num.src = "image/bomb_num.png";

    // Pow_player_speed
    res.player_speed.width = 50;
    res.player_speed.height = 50;
    res.player_speed.src = "image/player_speed.png";

    // SBlock
    res.sblock.width = 50;
    res.sblock.height = 50;
    res.sblock.src = "image/sblock.png";

    // Wall
    res.wall.width = 50;
    res.wall.height = 50;
    res.wall.src = "image/wall.png";

    // Bomb
    res.bomb.width = 50;
    res.bomb.height = 50;
    res.bomb.src = "image/bomb.png";

    // Fire
    res.fire.width = 50;
    res.fire.height = 50;
    res.fire.src = "image/fire.jpg";

    return res;
}

// Returns an object containing functions that will be linked to our wasm model
// This means that they can be called from Rust
const res = resources();

export class Draw {
    width(x) {
        //        canvas.width = window.innerWidth * 0.8;
        canvas.width = x;
        return canvas.width;
    }

    height(y) {
        //        canvas.height = window.innerHeight * 0.8;
        canvas.height = y;
        return canvas.height;
    }

    clear_screen() {
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    draw_player(x, y, angle, alive) {
        if (!alive) {
            return;
        }
        ctx.translate(x, y);
        ctx.rotate(angle);
        ctx.translate(-25, -25);
        ctx.drawImage(res.player, 0, 0, 50, 50);
//        ctx.rotate(angle);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_pow(x, y, pow_content) {
        if(pow_content == 0) {
            ctx.translate(x, y);
            ctx.translate(-25, -25);
            ctx.drawImage(res.bomb_power, 0, 0, 50, 50);
            ctx.setTransform(1, 0, 0, 1, 0, 0);
            ctx.fillStyle = "black";
        } else if(pow_content == 1) {
            ctx.translate(x, y);
            ctx.translate(-25, -25);
            ctx.drawImage(res.bomb_num, 0, 0, 50, 50);
            ctx.setTransform(1, 0, 0, 1, 0, 0);
            ctx.fillStyle = "black";
        } else if(pow_content == 2){
            ctx.translate(x, y);
            ctx.translate(-25, -25);
            ctx.drawImage(res.player_speed, 0, 0, 50, 50);
            ctx.setTransform(1, 0, 0, 1, 0, 0);
            ctx.fillStyle = "black";
        }
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_sblock(x, y) {
        ctx.translate(x, y);
        ctx.translate(-25, -25);
        ctx.drawImage(res.sblock, 0, 0, 50, 50);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_wall(x, y) {
        ctx.translate(x, y);
        ctx.translate(-25, -25);
        ctx.drawImage(res.wall, 0, 0, 50, 50);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_bomb(x, y) {
        ctx.translate(x, y);
        ctx.translate(-25, -25);
        ctx.drawImage(res.bomb, 0, 0, 50, 50);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }
    draw_fire(x, y) {
        ctx.translate(x, y);
        ctx.translate(-25, -25);
        ctx.drawImage(res.fire, 0, 0, 50, 50);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }


    // The real loading and running of our wasm starts here
    //let imports = { clear_screen, draw_player, draw_enemy, draw_bullet, draw_particle, draw_score };
    //imports.Math_atan = Math.atan;
    //imports.sin = Math.sin;
    //imports.cos = Math.cos;

}
