use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

// TODO use this.
mod project;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("RΞBLCK", "us")
        .build()
        .expect("Failed to create context.");

    let game = Reblck::new(&mut ctx);

    event::run(ctx, event_loop, game);
}

struct Reblck {

}

impl Reblck {
    pub fn new(_ctx: &mut Context) -> Reblck {
        Reblck {

        }
    }
}

impl EventHandler for Reblck {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        graphics::present(ctx)
    }
}