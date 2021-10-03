use chrono::{Datelike, Local};
use colored::Colorize;
use koyomi::{num_days, Calendar, Date};

// カレンダー作成時に指定する、開始日と終了日をもつ期間という意味の構造体を定義
struct Period {
    start_date: Date,
    end_date: Date,
}

// Period構造体に自身を作成するnewという関連関数を定義
impl Period {
    fn new() -> Period {
        // 現在の日時を取得
        let now = Local::now();
        let (year, month) = (now.year(), now.month());

        // 実行した月を対象とする 2021-10-01 と 2021-10-31 のような形式になる
        let start_date: String = format!("{}-{}-01", year.to_string(), month.to_string(),);
        let end_date: String = format!(
            "{}-{}-{}",
            year.to_string(),
            month.to_string(),
            // 指定した年月の最終日を返す
            num_days(year, month)
        );

        // 日付の文字列をDate型に変換
        let start_date = Date::parse(&start_date).unwrap();
        let end_date = Date::parse(&end_date).unwrap();

        // 自身を返却
        Period {
            start_date,
            end_date,
        }
    }
}

fn main() {
    // 実行した日時、時刻を取得
    let period = Period::new();

    // カレンダーを実行した月の期間で作成
    let calendar = match Calendar::new(period.start_date, period.end_date) {
        Ok(calendar) => calendar.make(),
        Err(err) => panic!("カレンダー初期化エラー {:?}", err),
    };

    // calendarはIterableなのでforで回し、日付を受け取るj
    for day in &calendar {
        // 曜日を判定し、土日の場合は赤・青で出力し、それ以外はスルー
        // day.weekday().japanese()はchar型を返すため、''で判定
        match day.weekday().japanese() {
            '土' => println!("{} 土", day.to_string().red()),
            '日' => println!("{} 日", day.to_string().blue()),
            // _は、上記以外の全てという意味のワイルドカード
            _ => {}
        }
        // 祝日の場合は、祝日の内容を緑で出力
        if day.holiday() != None {
            println!("{} {}", day.to_string().green(), day.holiday().unwrap(),);
        }

        // TODO: 土日と祝日がかぶっている場合の対処
    }
}

// TODO: テストコードの追加
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
