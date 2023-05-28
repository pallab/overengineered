pub struct Letters;

impl Letters {
    pub fn get(&self, c: char) -> Vec<(i32, i32)> {
        match c {
            'H' => Letters::H(),
            'W' => Letters::W(),
            _ => vec![]
        }
    }

    fn H() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b = vec![(3, 5), (4, 5), (5, 5)];
        a.append(&mut b);
        a
    }

    fn W() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b = vec![(3, 7), (4, 6), (5, 7)];
        a.append(&mut b);
        a
    }
}