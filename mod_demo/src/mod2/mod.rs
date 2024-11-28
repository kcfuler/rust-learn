mod sub_mod1;

use sub_mod1::sub_mod1_func;
pub fn mod2_func() {
    sub_mod1_func();
}
