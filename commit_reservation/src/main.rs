mod services;
mod domain;

use services::timer;
fn main() {
    timer::run();
}

#[cfg(test)]
mod test;