use bevy::math::Vec2;

pub enum Collider {
    Rectangle(Vec2),
    Circle(f32),
}

impl Collider {
    pub fn contains_point(&self, collider_location: &Vec2, point: &Vec2) -> bool {
        match self {
            Collider::Rectangle(size) => {
                let range = *size / 2.0;
                let (lows, highs) = (*collider_location - range, *collider_location + range);
                point.cmple(highs).all() && point.cmpge(lows).all()
            }
            Collider::Circle(radius) => todo!(),
        }
    }
}
