use crate::nodes::prelude::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameNode {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for GameNode {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let callable = self
            .base_mut()
            .callable("player_did_collide_with_static_body");

        let mut player = self.base_mut().get_node_as::<Player>("Player");
        player.connect("on_player_did_collide", &callable);
    }
}

#[godot_api]
impl GameNode {
    #[func]
    fn player_did_collide_with_static_body(&mut self) {
        let player = self.base_mut().get_node_or_null("Player").unwrap();
        self.base_mut().remove_child(&player);
    }
}
