fn main() {
    // https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics
    let days = [
        ("first", "A partridge in a pear tree"),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[day].0
        );

        for gift_day in (0..(day + 1)).rev() {
            let and = if gift_day == 1 { ", and" } else { "" };
            println!("{}{}", days[gift_day].1, and);
        }

        println!("");
    }
}
