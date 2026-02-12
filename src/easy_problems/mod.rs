pub mod intro;
use intro::sum_one_array;
use intro::maximum_wealth;
use intro::fizz_buzz;

pub fn intro_execution_workspace() {
    let nums = vec![1,2,3,4];
    let ret = sum_one_array(nums);

    assert_eq!(3, ret[1]);

    let nums = vec!{
        vec![1,2,3],
        vec![3,2,1],
    };
    let n = maximum_wealth(nums);
    assert_eq!(n, 6);

    let nums = vec!{
        vec![1,5],
        vec![7,3],
        vec![3,5],
    };

    let n = maximum_wealth(nums);
    assert_eq!(n, 10);


    fizz_buzz(3);
}