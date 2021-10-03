use chrono::Local;
use std::env::args;

// 練習してただけのコード↓
fn main() {
  // 実行した日時、時刻を取得
  let now = Local::now();
  println!("now {}", &now);

  // 実行した日時、時刻を取得
  let now = Local::now();
  println!("now {}", &now);

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

// その月の日付をすべて出力
fn _print_one_month_days(calendar: std::vec::Vec<koyomi::Date>) {
  // コマンドライン引数にallがあれば、全日付を出力
  // TODO: エラーハンドリング
  let args: Vec<String> = args().collect();
  let flag = &args[1];
  if flag != "all" {}
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
