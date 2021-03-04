use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;


pub struct Object<'a> {
    pub texture: Texture<'a>,
    pub src: Rect,
    pub pos: Rect,
}


impl Object<'_> {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.copy_ex(
            &self.texture,
            Some(self.src),
            Some(self.pos),
            0.0,
            None,
            false,
            false,
        ).unwrap();
    }
}


pub struct Agent<'a> {
    pub id: i32,
    pub texture: Texture<'a>,
    pub src: Rect,
    // Note: pos.width and pos.height are actual sizes of the agent.
    pub pos: Rect,
    pub name: Texture<'a>,
    // speed that is used to set velocity
    pub speed: i32,
    // Vertical and horizontal velocity
    pub vv: i32,
    pub hv: i32,
    pub is_left: bool,
    pub is_jump: bool,
}


impl Agent<'_> {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // Drawing main texture
        canvas.copy_ex(
            &self.texture,
            Some(self.src),
            Some(self.pos),
            0.0,
            None,
            self.is_left,
            false,
        ).unwrap();
        // Drawing display name
        let t = &self.name.query();
        let offset: i32;
        if self.is_left { offset = -5; } else { offset = 5; }
        let pos = Rect::new(
            self.pos.x + ((self.pos.width() - t.width) / 2) as i32 - offset,
            self.pos.y - 25, t.width, t.height
        );
        canvas.copy_ex(
            &self.name,
            None,
            Some(pos),
            0.0,
            None,
            false,
            false,
        ).unwrap();
    }
}


pub struct GameContext<'a> {
    pub canvas:     &'a mut Canvas<Window>,

    pub window_w:   u32,
    pub window_h:   u32,
    pub gravity:    i32,
    pub max_vv:     i32,

    pub players:    Vec<Agent<'a>>,
    pub platform:   Object<'a>,
    pub background: Object<'a>,
    pub running:    bool,
}


impl GameContext<'_> {
    pub fn render_scene(&mut self) {
        // Drawing background
        self.background.draw(&mut self.canvas);
        // Drawing platform
        self.platform.draw(&mut self.canvas);
        // Render all players
        for player in &mut self.players {
            player.draw(&mut self.canvas);
        }
    }

    pub fn apply_physics(&mut self) {
        for pi in 0..self.players.len() {
            {  // Borrow player here
                let player = &mut self.players[pi];
                // Apply gravity
                player.vv -= self.gravity;

                // Limit maximum velocity, because on extreme speeds (which are very easy to get with decent height)
                // player will teleport through objects like plaform and fall into the abyss.
                if player.vv > self.max_vv {
                    player.vv = self.max_vv;
                }
                if player.vv < -self.max_vv {
                    player.vv = -self.max_vv;
                }
            }  // End player borrow

            for oi in 0..self.players.len() {
                // Avoid collision in the same object
                if pi == oi { continue; }
                if !self.players[pi].pos.has_intersection(self.players[oi].pos) { continue; }

                // Here we know we have collision

                // println!("{:#?}", self.players[pi].pos.intersection(self.players[oi].pos).unwrap());
                if self.players[pi].pos.x() > self.players[oi].pos.x() {
                    self.players[pi].hv += self.players[oi].hv;
                } else {
                    self.players[pi].hv -= self.players[oi].hv;
                }

                // This doesn't work
                let px = self.players[pi].pos.x();
                let py = self.players[pi].pos.y();
                let pw = self.players[pi].pos.width() as i32;
                let ph = self.players[pi].pos.height() as i32;
                let ox = self.players[oi].pos.x();
                let oy = self.players[oi].pos.y();
                let ow = self.players[oi].pos.width() as i32;

                if (py + ph) >= oy && ((px > ox && (ox + ow) < px) || ((px + pw) > ox && (ox + ow) < (px + pw))) {
                    // delta_y = delta;
                    if self.players[pi].is_jump { self.players[pi].is_jump = false; }
                    self.players[pi].vv = 0;
                    let new_y = oy - ph;
                    self.players[pi].pos.set_y(new_y);
                }/* else {
                    delta_y += 10;
                }*/
            }
            // Finally we apply movement
            {  // Borrow player here
                let player = &mut self.players[pi];
                // Horizontal movement
                let new_pos: f32 = (player.pos.x() + player.hv) as f32;
                // Limit movement out of the srceen
                if new_pos > ((0 as f32) - (player.pos.width() as f32) * 0.3) && new_pos < ((self.window_w as f32) - (player.pos.width() as f32) * 0.7) {
                    player.hv = 0;
                    player.pos.set_x(new_pos as i32);
                }
                // Vertical movement
                player.pos.set_y(player.pos.y() - player.vv);

                // TODO: should be a part of a proper object collision
                if self.platform.pos.has_intersection(player.pos) {
                    if player.is_jump { player.is_jump = false; }
                    player.vv = 0;
                    player.pos.set_y(self.platform.pos.y() - (player.pos.height() as i32));
                }
            }  // End player borrow
        }
    }
}

