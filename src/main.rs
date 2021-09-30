use chrono::{Datelike, Local, NaiveDate};
use koyomi::{Calendar, Date};

fn main() {
    // 実行した日時、時刻を取得
    let now = Local::now();
    println!("now {}", &now);

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

    // 開始日時
    let start_date = NaiveDate::from_ymd(now.year(), now.month(), 1);
    println!("start_date {}", start_date);

    let date = Date::parse("2021-09-01").unwrap();
    let date2 = Date::parse("2021-09-30").unwrap();

    let calendar = Calendar::new(date, date2).unwrap().make();
    for day in calendar {
        println!("calendar days {} {}", day.day(), day.weekday().japanese(),);
    }

    // 現在日時をフォーマット
    let now = now.format("%Y年 %m月 %d日");
    println!("now -> {}", now);
}

// TODO: test
