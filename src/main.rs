mod print;
mod vars;
mod check_if_prime;
mod prime_checker_v2;
mod hello_world;

fn main() {
    let num: i128 = 11;
    let option:i128 = 1;
    let name = "John";
    hello_world::run(name);
    print::run();
    vars::run();
    check_if_prime::run(num);
    prime_checker_v2::run(option, num);
}
