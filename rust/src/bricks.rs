use godot::prelude::*;

use crate::brick::Brick;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Bricks {
    base: Base<Node2D>,

    #[export]
    brick: OnEditor<Gd<PackedScene>>,

    count: i32,
}

#[godot_api]
impl Bricks {
    #[signal]
    pub fn bricks_cleared();

    pub fn generate(&mut self) {
        self.count = 0;

        let gap = 4.28;

        for i in 0..3 {
            let y = (32.0 + gap) * i as f32 + 16.0 + 10.0;

            for j in 0..15 {
                let x = (80.0 + gap) * j as f32 + 40.0 + 10.0;

                let mut brick = self.brick.instantiate_as::<Brick>();
                brick.set_position(Vector2::new(x, y));
                brick
                    .signals()
                    .hitted()
                    .connect_other(&*self, Self::on_brick_hited);

                self.base_mut().add_child(&brick);
                self.count += 1;
            }
        }
    }

    pub fn queue_free_all_bricks(&mut self) {
        self.base()
            .get_tree()
            .call_group("bricks", "queue_free", &[]);
    }

    fn on_brick_hited(&mut self) {
        self.count -= 1;
        if self.count == 0 {
            self.signals().bricks_cleared().emit();
        }
    }
}
