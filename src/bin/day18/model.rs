#[derive(Debug, PartialEq)]
pub struct DropPart {
    x: i32,
    y: i32,
    z: i32,
}

impl DropPart {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn get_empty_side(&self, other: &[DropPart]) -> i32 {
        6 - self.get_neighbor_count(other)
    }

    fn get_empty_droplets(&self, other: &[DropPart]) -> Vec<DropPart> {
        let mut empty_droplets = Vec::new();
        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    let droplet = DropPart::new(x, y, z);
                    if !other.contains(&droplet) {
                        empty_droplets.push(droplet);
                    }
                }
            }
        }
        empty_droplets
    }

    fn is_enclosed(&self, other: &[DropPart], max_x: i32, max_y: i32, max_z: i32) -> bool {
        for x in self.x+1..=max_x {
            let droplet = DropPart::new(x, self.y, self.z);
            if other.contains(&droplet){
                return true;
            }
        }
        for y in self.y+1..=max_y {
            let droplet = DropPart::new(self.x, y, self.z);
            if other.contains(&droplet){
                return true;
            }
        }
        for z in self.z+1..=max_z {
            let droplet = DropPart::new(self.x, self.y, z);
            if other.contains(&droplet){
                return true;
            }
        }
        false
    }

    fn get_neighbor_count(&self, other: &[DropPart]) -> i32 {
        other.iter().filter(|drop| self.is_neighbor(drop)).count() as i32
    }

    fn is_neighbor(&self, other: &DropPart) -> bool {
        let x_diff = (self.x as i32 - other.x as i32).abs();
        let y_diff = (self.y as i32 - other.y as i32).abs();
        let z_diff = (self.z as i32 - other.z as i32).abs();
        x_diff + y_diff + z_diff == 1
    }
}

pub fn max_dimension(droplets: &[DropPart]) -> (i32, i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for droplet in droplets {
        if droplet.x > max_x {
            max_x = droplet.x;
        }
        if droplet.y > max_y {
            max_y = droplet.y;
        }
        if droplet.z > max_z {
            max_z = droplet.z;
        }
    }
    (max_x, max_y, max_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_get_neighbor_count() {
        let drop = DropPart::new(0, 0, 0);
        let others = vec![
            DropPart::new(0, 0, 1),
            DropPart::new(0, 1, 0),
            DropPart::new(1, 0, 0),
            DropPart::new(1, 1, 1),
        ];
        assert_eq!(drop.get_neighbor_count(&others), 3);
    }

    #[test]
    fn could_get_max_dimension() {
        let droplets = vec![
            DropPart::new(0, 0, 1),
            DropPart::new(0, 1, 0),
            DropPart::new(1, 0, 0),
            DropPart::new(1, 1, 1),
        ];
        assert_eq!(max_dimension(&droplets), (1, 1, 1));
    }
}
