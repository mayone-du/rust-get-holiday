use chrono::Local;

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
}
