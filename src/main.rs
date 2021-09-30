use chrono::{Datelike, Local};
use koyomi::{num_days, Calendar, Date};

fn main() {
    // 実行した日時、時刻を取得
    let now = Local::now();

    let year = now.year();
    let month = now.month();

    // 実行した月を対象とする
    let start_date: String = format!("{}-{}-01", year.to_string(), month.to_string(),);
    let end_date: String = format!(
        "{}-{}-{}",
        year.to_string(),
        month.to_string(),
        // 指定した年月の最終日を返す
        num_days(year, month)
    );

    // 開始日時
    let start_date = Date::parse(&start_date).unwrap();
    let end_date = Date::parse(&end_date).unwrap();

    // カレンダーを取得
    let calendar = Calendar::new(start_date, end_date).unwrap().make();
    for day in calendar {
        // 全日にちを出力
        // println!(
        //     "calendar days {} {} {:?}",
        //     day.day(),
        //     // 曜日
        //     day.weekday().japanese(),
        //     // 祝日
        //     day.holiday(),
        // );

        // 土日の場合
        match day.weekday().japanese() {
            '土' => println!("土曜 {}", day),
            '日' => println!("日曜 {}", day),
            // 土日以外
            _ => {}
        }
        if day.holiday() != None {
            println!("{} {}", day.holiday().unwrap(), day);
        }
    }

    // 現在日時をフォーマット
    let now = now.format("%Y年 %m月 %d日");
    println!("now -> {}", now);
}

// TODO: test
