// xfail-stage0

pred even(uint x) -> bool {
  if (x < 2) {
    ret false;
  }
  else if (x == 2) {
    ret true;
  }
  else {
    ret even(x - 2);
  }
}

fn foo(uint x) -> () {
  if check(even(x)) { 
      log x;
    }
  else {
    fail;
  }
}

fn main() {
  foo(2u);
}
