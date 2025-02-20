pub fn find_max_prime_factor(number: u128) -> u128 {
    if number == 0 {
        return 0;
    }
    if number == 1 {
        return 1;
    }

    let mut max_factor = 1;
    let mut n = number;

    // Handle factor 2
    if n % 2 == 0 {
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    if n == 1 {
        return max_factor;
    }

    // Factorize the remaining part and find the maximum prime factor
    let factors = factorize(n);
    factors.into_iter().max().unwrap_or(max_factor)
}

fn factorize(mut n: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    // Trial division for small primes
    for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == 1 {
            break;
        }
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }

    if n == 1 {
        return factors;
    }

    // Check if remaining n is a prime
    if is_prime(n) {
        factors.push(n);
        return factors;
    }

    // Use Pollard's Rho algorithm to find a divisor
    let d = pollards_rho_brent(n);
    let mut factors_d = factorize(d);
    let mut factors_n = factorize(n / d);
    factors.append(&mut factors_d);
    factors.append(&mut factors_n);
    factors
}

fn pollards_rho_brent(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }

    let mut c = 1;
    loop {
        let mut y = 2;
        let mut x = y;
        let m = 128;
        let mut g = 1;
        let mut r = 1;
        let mut q = 1;

        while g == 1 {
            x = y;
            for _ in 0..r {
                y = (mod_mul(y, y, n) + c) % n;
            }
            let mut k = 0;
            while k < r && g == 1 {
                let mut ys = y;
                let bound = std::cmp::min(m, r - k);
                for _ in 0..bound {
                    y = (mod_mul(y, y, n) + c) % n;
                    q = mod_mul(q, (x as i128 - y as i128).abs() as u128, n);
                }
                g = gcd(q, n);
                k += bound;
            }
            r *= 2;
        }

        if g != n {
            return g;
        }

        // Try next c if current fails
        c += 1;
        if c > 5 {
            // Fallback to trial division for small c values exhausted
            for d in 2..=5 {
                if n % d == 0 {
                    return d;
                }
            }
            // If all else fails, return n (though n should be composite)
            return n;
        }
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // Bases for deterministic test for n < 2^64
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut found = false;
        for _ in 0..s - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
    let mut result = 1;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp /= 2;
    }
    result
}

fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b % m;
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    result
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}