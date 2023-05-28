use rand::random;

pub struct Letters;

impl Letters {
    pub fn get(&self, c: char) -> Vec<(i32, i32)> {
        match c {
            'H' => Letters::h(),
            'E' => Letters::e(),
            'L' => Letters::l(),
            'O' => Letters::o(),
            'W' => Letters::w(),
            'R' => Letters::r(),
            'D' => Letters::d(),
            '!' => Letters::exclamation(),
            _ => Letters::random_pattrn()
        }
    }

    fn h() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b = vec![(3, 5), (4, 5), (5, 5)];
        a.append(&mut b);
        a
    }
    fn e() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y)]
        }).collect();

        let mut b: Vec<(i32, i32)> = (3..6).flat_map(|x| {
            vec![(x, 2), (x, 5), (x, 8)]
        }).collect();

        a.append(&mut b);
        a
    }
    fn l() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y)]
        }).collect();

        let mut b = vec![(3, 8), (4, 8), (5, 8)];
        a.append(&mut b);
        a
    }
    fn o() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (3..8).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b: Vec<(i32, i32)> = (3..6).flat_map(|x| {
            vec![(x, 2), (x, 8)]
        }).collect();
        a.append(&mut b);
        a
    }

    fn w() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b = vec![(3, 7), (4, 6), (5, 7)];
        a.append(&mut b);
        a
    }
    fn r() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..9).flat_map(|y| {
            vec![(2, y)]
        }).collect();

        let mut b: Vec<(i32, i32)> = (3..6).flat_map(|x| {
            vec![(x, 2), (x, 5)]
        }).collect();

        let mut c = vec![(5, 3), (5, 4), (3, 6), (4, 7), (5, 8)];

        a.append(&mut b);
        a.append(&mut c);

        a
    }
    fn d() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (3..8).flat_map(|y| {
            vec![(2, y), (6, y)]
        }).collect();

        let mut b: Vec<(i32, i32)> = (2..6).flat_map(|x| {
            vec![(x, 2), (x, 8)]
        }).collect();
        a.append(&mut b);
        a
    }
    fn exclamation() -> Vec<(i32, i32)> {
        let mut a: Vec<(i32, i32)> = (2..7).flat_map(|y| {
            vec![(4, y)]
        }).collect();

        let mut b = vec![(4, 8)];
        a.append(&mut b);
        a
    }

    fn random_pattrn() -> Vec<(i32, i32)> {
        (2..9).flat_map(|y| (2..8).flat_map( move |x| {
            if rand::random() { vec![(x, y)] } else { vec![] }
        })).collect()
    }
}