use chrono::{Datelike, Local};
use koyomi::{Calendar, Date};

fn main() {
    // 実行した日時、時刻を取得
    let now = Local::now();
    println!("now {}", &now);

    // 実行した月を対象とする
    // TODO: 本当に使うなら、実行した次の月とか、引数から月を選択できるといいかも
    let start_date: String = format!("{}-{}-01", now.year().to_string(), now.month().to_string(),);
    // TODO: その月の最後の日付
    let end_date: String = format!("{}-{}-31", now.year().to_string(), now.month().to_string(),);

    // 開始日時
    let start_date = Date::parse(&start_date).unwrap();
    let end_date = Date::parse(&end_date).unwrap();

    // カレンダーを取得
    let calendar = Calendar::new(start_date, end_date).unwrap().make();
    for day in calendar {
        println!(
            "calendar days {} {} {:?}",
            day.day(),
            // 曜日
            day.weekday().japanese(),
            // 祝日
            day.holiday(),
        );
    }

    // 現在日時をフォーマット
    let now = now.format("%Y年 %m月 %d日");
    println!("now -> {}", now);
}

// TODO: test
