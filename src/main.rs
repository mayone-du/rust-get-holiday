use chrono::{Datelike, Local};
use colored::Colorize;
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
    // let start_date = match Date::parse(&start_date) {
    //     Ok(date) => date,
    //     Err(err) => panic!("エラー {:?}", err),
    // };
    let start_date = Date::parse(&start_date).unwrap();
    let end_date = Date::parse(&end_date).unwrap();

    // カレンダーを取得
    let calendar = match Calendar::new(start_date, end_date) {
        Ok(calendar) => calendar.make(),
        Err(err) => panic!("カレンダーエラー {:?}", err),
    };
    for day in &calendar {
        // 土日の場合
        match day.weekday().japanese() {
            '土' => println!("{} 土", day.to_string().red()),
            '日' => println!("{} 日", day.to_string().blue()),
            // 土日以外
            _ => {}
        }
        if day.holiday() != None {
            println!("{} {}", day.to_string().green(), day.holiday().unwrap(),);
        }
    }

    print_one_month_days(calendar);
}

fn print_one_month_days(calendar: std::vec::Vec<koyomi::Date>) {
    print!("\n");
    for day in calendar {
        // 全日にちを出力
        println!(
            "calendar days {} {} {:?}",
            day.day(),
            // 曜日
            day.weekday().japanese(),
            // 祝日
            day.holiday(),
        );
    }
}

// TODO: test
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn it_print_one_month_days() {
    //     // すべての日数を表示
    //     print_one_month_days(
    //         Calendar::new(
    //             koyomi::Date::parse("2021-09-01").unwrap(),
    //             koyomi::Date::parse("2021-09-30").unwrap(),
    //         )
    //         .unwrap()
    //         .make(),
    //     )
    // }
}
