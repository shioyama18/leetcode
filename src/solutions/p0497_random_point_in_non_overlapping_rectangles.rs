use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

struct Rectangle {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Rectangle {
    fn new(points: Vec<i32>) -> Self {
        assert_eq!(4, points.len());

        Self {
            x_min: points[0],
            x_max: points[2] + 1,
            y_min: points[1],
            y_max: points[3] + 1,
        }
    }

    fn area(&self) -> i32 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min)
    }

    fn pick(&self, rng: &mut ThreadRng) -> Vec<i32> {
        let x = rng.gen_range(self.x_min, self.x_max);
        let y = rng.gen_range(self.y_min, self.y_max);
        vec![x, y]
    }
}

struct Solution {
    rects: Vec<Rectangle>,
    weights: WeightedIndex<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let rects = rects.into_iter().map(Rectangle::new).collect::<Vec<_>>();
        let weights = rects.iter().map(Rectangle::area).collect::<Vec<_>>();

        Self {
            rects,
            weights: WeightedIndex::new(&weights).unwrap(),
            rng: rand::thread_rng(),
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        self.rects[self.weights.sample(&mut self.rng)].pick(&mut self.rng)
    }
}
