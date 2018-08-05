#[macro_use]
extern crate criterion;
extern crate rand;
extern crate test_work_price_list;

use criterion::Criterion;
use test_work_price_list::{PriceList, Size, Meta, Inner};


fn gen_size_meta(times: u32) -> Vec<(Size, Meta)> {
    (0..times).map(|_| (rand::random(), rand::random::<u64>() as u128)).collect()
}


fn bench_price_list_add(c: &mut Criterion) {



    c.bench_function("add", |b| {
        let mut inner: Inner = vec![];
        for _i in 0..1000 {
            inner.push((rand::random(), gen_size_meta(100)));
        }


        b.iter(|| {
            let mut price_list = PriceList::new();

            for i in &inner {
                for j in &i.1 {
                    price_list.add(i.0, *j);
                }
            }
        });
    });

}


fn bench_split(c: &mut Criterion) {

    c.bench_function("split", |b| {

        let mut inner: Inner = vec![];
        for _i in 0..1000 {
            inner.push((rand::random(), gen_size_meta(100)));
        }

        let mut price_list = PriceList::new();

        for i in &inner {
            for j in &i.1 {
                price_list.add(i.0, *j);
            }
        }

        b.iter(|| {
            price_list.split(rand::random(), rand::random());
        });

    });



}



criterion_group!(benches, bench_price_list_add, bench_split);
//criterion_group!(benches, bench_price_list_add);
criterion_main!(benches);