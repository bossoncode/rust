pub fn types(){

    //Default i32
    let _num = 69;

    //Default f64
    let _fl = 6.9;

    //Explicit type
    let _exp:i128 = 2345678987654325678909876;

    //Find max size
    println!("i32:{} | i64:{} | i128:{}", std::i32::MAX, std::i64::MAX, std::i128::MAX);

    //Boolean
    let _bool = true;

    let _is_active:bool = false;

    println!("{}", 2 > 6 );

    //Char
    let _f = 'f';
    let heart = '\u{2665}';

    println!("{}", heart);

}