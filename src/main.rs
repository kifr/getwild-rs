use rand::Rng;

fn main() {
    let arr = ["Get", "wild", "and", "tough"];
    let mut rng = rand::thread_rng();
    let mut v: Vec<&str> = vec![];
    let mut magic_words: String = "".to_string();
    let mut count = 0;

    loop {
        let i: usize = rng.gen_range(0, 4);

        if i == v.len() {v.push(arr[i]);}
        else {v = vec![];}

        count += 1;

        if v.len() != 0 {
            magic_words = format!("{} {}", magic_words, arr[i].to_string());
            println!("{}:{}", count, magic_words);
        } else {
            magic_words = "".to_string();
            println!("{}: {}", count, arr[i]);
        }

        if v.len() == 4 {break;}
    }

    let result_text = format!("\nYour recode is {}", count);

    if count      <=  20 {println!("{}!!! AWESOME!!!!🎉", result_text)}
    else if count <=  50 {println!("{}!! Great!😎", result_text)}
    else if count <= 100 {println!("{}! Good!👍", result_text)}
    else if count <= 200 {println!("{}! Not too bad.", result_text)}
    else                 {println!("{} 😅", result_text)}
}