use tokio::runtime;


pub fn thread_init() {
    let core_ids = core_affinity::get_core_ids().unwrap();
    println!("core num {}", core_ids.len());

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut idx = 2;

    rt.block_on(async {
        for i in 0..8 {
            println!("num {}", i);
            let core_id = core_ids[idx];
            if idx.eq(&(core_ids.len() - 1)) {
                idx = 2;
            } else {
                idx += 1;
            }

            tokio::spawn(async move {
                let res = core_affinity::set_for_current(core_id);
                println!("{}", res);
                loop {
                    let mut sum: i32 = 0;
                    for i in 0..100000000 {
                        sum = sum.overflowing_add(i).0;
                    }
                    println!("sum {}", sum);
                    }
            });
        }
    });
}