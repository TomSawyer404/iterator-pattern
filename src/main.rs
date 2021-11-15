trait MyIterator {
    type Object;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Self::Object;
}

#[derive(Debug, Clone, Copy)]
struct Good {
    price: usize,
}

impl Good {
    fn new(price: usize) -> Self {
        Good { price: (price) }
    }

    fn get_price(&self) -> usize {
        self.price
    }
}

#[derive(Debug, Clone)]
struct Store {
    goods: Vec<Good>,
    num_goods: usize,
}

impl Store {
    fn new(maxsize: usize) -> Self {
        Store {
            goods: Vec::with_capacity(maxsize),
            num_goods: 0,
        }
    }

    fn add_good(&mut self, good: Good) {
        self.goods.push(good);
        self.num_goods += 1;
    }

    fn get_num(&self) -> usize {
        self.num_goods
    }

    fn get_good_at(&self, index: usize) -> Good {
        self.goods[index].clone()
    }

    fn iterator(&self) -> Box<dyn MyIterator<Object = Good>> {
        Box::new(StoreIterator::new((*self).clone()))
    }
}

struct StoreIterator {
    store: Store,
    index: usize,
}

impl StoreIterator {
    fn new(store: Store) -> Self {
        StoreIterator { store, index: 0 }
    }
}

impl MyIterator for StoreIterator {
    type Object = Good;

    fn has_next(&self) -> bool {
        if self.index < self.store.get_num() {
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Self::Object {
        let good = self.store.get_good_at(self.index);
        self.index += 1;
        good
    }
}

fn main() {
    let mut store_chunxilu = Store::new(8);
    store_chunxilu.add_good(Good::new(12));
    store_chunxilu.add_good(Good::new(28));
    store_chunxilu.add_good(Good::new(49));
    store_chunxilu.add_good(Good::new(73));

    let mut store_iterator = store_chunxilu.iterator();
    while store_iterator.has_next() {
        let good = store_iterator.next();
        println!("{:?}", good.get_price());
    }
}
