trait Entity {
    fn position(&self) -> (f32, f32, f32);
    fn distance_from(&self, other: &impl Entity) -> f32 {
        let (x1, y1, z1) = self.position();
        let (x2, y2, z2) = other.position();
        ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
    }
    fn close_to_me<'a, E>(
        &'a self,
        others: impl Iterator<Item = &'a E> + 'a,
        dist: f32,
    ) -> impl Iterator<Item = &'a E> + 'a
    where
        E: Entity + 'a,
    {
        others.filter(move |&other| self.distance_from(other) < dist)
    }
}

fn close_to_entity<'a, E1, E2>(
    entity: &'a E1,
    others: impl Iterator<Item = &'a E2> + 'a,
    dist: f32,
) -> impl Iterator<Item = &'a E2> + 'a
where
    E1: Entity,
    E2: Entity + 'a,
{
    others.filter(move |&other| entity.distance_from(other) < dist)
}

impl Entity for (f32, f32, f32) {
    fn position(&self) -> (f32, f32, f32) {
        *self
    }
}
impl Entity for &(f32, f32, f32) {
    fn position(&self) -> (f32, f32, f32) {
        **self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let point1 = (1.0, 2.0, 3.0);
        let point2 = (4.0, 5.0, 6.0);

        assert!((point1.distance_from(&point2) - 5.196152422706632).abs() < 1e-6);
    }

    #[test]
    fn test_close_to_me() {
        let point1 = (1.0, 2.0, 3.0);
        let point2 = (4.0, 5.0, 6.0);
        let point3 = (1.0, 2.0, 7.0);
        let points = [&point2, &point3];

        let close_points: Vec<_> = point1.close_to_me(points.iter(), 5.0).collect();
        assert_eq!(close_points.len(), 1);
    }
}
