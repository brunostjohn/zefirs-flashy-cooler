macro_rules! from_ul_arr {
    ($arr:expr, $from:ident) => {
        [
            $arr[0].$from,
            $arr[1].$from,
            $arr[2].$from,
            $arr[3].$from,
            $arr[4].$from,
            $arr[5].$from,
            $arr[6].$from,
            $arr[7].$from,
        ]
    };
    (mat $arr:expr, $from:ident) => {
        [
            from_ul_arr!(mat $arr[0].$from),
            from_ul_arr!(mat $arr[1].$from),
            from_ul_arr!(mat $arr[2].$from),
            from_ul_arr!(mat $arr[3].$from),
            from_ul_arr!(mat $arr[4].$from),
            from_ul_arr!(mat $arr[5].$from),
            from_ul_arr!(mat $arr[6].$from),
            from_ul_arr!(mat $arr[7].$from),
        ]
    };
    (mat $arr: expr) => {
        [
            [$arr[0], $arr[1], $arr[2], $arr[3]],
            [$arr[4], $arr[5], $arr[6], $arr[7]],
            [$arr[8], $arr[9], $arr[10], $arr[11]],
            [$arr[12], $arr[13], $arr[14], $arr[15]],
        ]
    };
}

pub(crate) use from_ul_arr;
