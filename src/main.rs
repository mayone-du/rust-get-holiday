use chrono::{Datelike, Local};

fn main() {
    // 実行した日時、時刻を取得
    let now = Local::now();

    // 実行したタイミングの月を取得
    let month = now.month();
    println!("month {}", month);

    // match式で曜日を日本語に変換
    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "月",
        chrono::Weekday::Tue => "火",
        chrono::Weekday::Wed => "水",
        chrono::Weekday::Thu => "木",
        chrono::Weekday::Fri => "金",
        chrono::Weekday::Sat => "土",
        chrono::Weekday::Sun => "日",
    };
    println!("{}", weekday);

    // 現在日時をフォーマット
    let now = now.format("%Y年 %m月 %d日");
    println!("now -> {}", now);
}
