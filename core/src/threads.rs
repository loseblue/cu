use tokio::runtime;


pub fn thread_init() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let core_ids_map = [2, 5, 6, 8];

    rt.block_on(async {
        for idx in &core_ids_map {
            
            println!("core_id = {}", idx);
            let core_id = core_ids[*idx];

            tokio::spawn(async move {
                let res = core_affinity::set_for_current(core_id);
                loop {
                    let mut sum: i32 = 0;
                    for i in 0..100000000 {
                        sum = sum + i;
                        println!("sum {}", sum);
                    }
                    }
            });
        }
    });
}