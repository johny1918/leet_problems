pub mod intro;
use intro::sum_one_array;

pub fn intro_execution_workspace() {
    let nums = vec![1,2,3,4];
    let ret = sum_one_array(nums);

    assert_eq!(3, ret[1]);
}