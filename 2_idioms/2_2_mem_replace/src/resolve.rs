use crate::{Solver, Trinity};



impl<T: Clone> Trinity<T> {
    pub fn rotate2(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
        std::mem::swap(&mut self.b, &mut self.c);
    }
}



impl<T: Clone + PartialEq> Solver<T> {
    pub fn resolve2(&mut self) {
        let expected = &self.expected;

        let old = std::mem::take(&mut self.unsolved);
        let mut unsolved = Vec::with_capacity(old.len());

        'l: for mut t in old {
            for _ in 0..3 {
                if t == *expected {
                    continue 'l;
                }
                t.rotate2();
            }
            unsolved.push(t);
        }

        self.unsolved = unsolved;
    }
}
