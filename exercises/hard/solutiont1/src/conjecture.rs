// conjecture.rs
pub fn goldbach_conjecture() -> String {
    const MAX_NUM: usize = 1_000_000;
    let is_prime = sieve(MAX_NUM);

    let mut results = Vec::new();
    let mut n = 9;

    while results.len() < 2 {
        if n > MAX_NUM {
            panic!("n exceeded MAX_NUM, consider increasing MAX_NUM");
        }

        // 跳过素数，只处理奇合数
        if is_prime[n] {
            n += 2;
            continue;
        }

        let k_max = ((n - 2) as f64 / 2.0).sqrt() as u64;
        let mut found = false;

        // 检查所有可能的k值
        for k in 1..=k_max {
            let p = n as u64 - 2 * k * k;
            if p < 2 {
                continue;  // 素数必须≥2
            }
            if p > MAX_NUM as u64 {
                panic!("p exceeded MAX_NUM, consider increasing MAX_NUM");
            }
            if is_prime[p as usize] {
                found = true;
                break;
            }
        }

        // 记录无法表示的数字
        if !found {
            results.push(n.to_string());
        }

        n += 2;  // 检查下一个奇数
    }

    results.join(",")
}

// 优化的埃拉托斯特尼筛法
fn sieve(max_num: usize) -> Vec<bool> {
    let mut is_prime = vec![true; max_num + 1];
    is_prime[0] = false; // 0不是素数
    if max_num >= 1 {
        is_prime[1] = false; // 1不是素数
    }
    for i in 2..=(max_num as f64).sqrt() as usize {
        if is_prime[i] {
            is_prime[i*i..=max_num].iter_mut()
                .step_by(i)
                .for_each(|x| *x = false);
        }
    }
    is_prime
}