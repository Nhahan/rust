// Project: Interaction bill manager

struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

fn main() {

}
