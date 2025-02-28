use dialoguer::{Confirm, FuzzySelect, Input};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

const FILE_PATH: &str = "travel_plan.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Trip {
    departure_time: String,
    departure_station: String,
    line: String,
    train_type: String,
    destination: String,
    fare: u32,
    arrival_time: String,
    arrival_station: String,
}

fn main() {
    loop {
        println!("\n旅行計画管理システム");
        let options = vec![
            "移動計画を追加",
            "移動計画を編集",
            "保存された計画を表示",
            "終了",
        ];
        let selection = FuzzySelect::new()
            .with_prompt("操作を選択してください")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => add_trip(),   // 移動計画を追加
            1 => edit_trip(),  // 移動計画を編集
            2 => list_trips(), // 保存された計画を表示
            3 => {
                println!("終了します");
                break; // 終了
            }
            _ => (),
        }
    }
}

fn add_trip() {
    let departure_time: String = Input::new()
        .with_prompt("出発時刻 (HH:MM)")
        .interact_text()
        .unwrap();

    let departure_station: String = Input::new().with_prompt("出発駅").interact_text().unwrap();

    let line: String = Input::new().with_prompt("路線").interact_text().unwrap();

    let train_type: String = Input::new()
        .with_prompt("列車種別")
        .interact_text()
        .unwrap();

    let destination: String = Input::new().with_prompt("行先").interact_text().unwrap();

    let fare: u32 = Input::new().with_prompt("料金").interact_text().unwrap();

    let arrival_time: String = Input::new()
        .with_prompt("到着時刻 (HH:MM)")
        .interact_text()
        .unwrap();

    let arrival_station: String = Input::new().with_prompt("到着駅").interact_text().unwrap();

    let trip = Trip {
        departure_time,
        departure_station,
        line,
        train_type,
        destination,
        fare,
        arrival_time,
        arrival_station,
    };

    let mut trips = load_trips();
    trips.push(trip);
    save_trips(&trips);
    println!("データを保存しました。");
}

fn edit_trip() {
    let trips = load_trips();
    if trips.is_empty() {
        println!("保存された旅行計画はありません。");
        return;
    }

    let trip_names: Vec<String> = trips
        .iter()
        .enumerate()
        .map(|(i, trip)| {
            format!(
                "{}. {} → {}",
                i + 1,
                trip.departure_station,
                trip.arrival_station
            )
        })
        .collect();

    let selected_trip_index = FuzzySelect::new()
        .with_prompt("編集する移動計画を選んでください")
        .items(&trip_names)
        .default(0)
        .interact()
        .unwrap();

    // 直接インデックスを使用
    let trip_index = selected_trip_index; // ここでsplitは不要です

    let mut trip = trips[trip_index].clone();

    // 編集メニュー
    loop {
        let edit_options = vec![
            "出発時刻を編集",
            "出発駅を編集",
            "路線を編集",
            "列車種別を編集",
            "行先を編集",
            "料金を編集",
            "到着時刻を編集",
            "到着駅を編集",
            "削除する",
            "終了",
        ];
        let edit_selection = FuzzySelect::new()
            .with_prompt("編集する項目を選んでください")
            .items(&edit_options)
            .default(0)
            .interact()
            .unwrap();

        match edit_selection {
            0 => {
                let departure_time: String = Input::new()
                    .with_prompt("新しい出発時刻 (HH:MM)")
                    .interact_text()
                    .unwrap();
                trip.departure_time = departure_time;
            }
            1 => {
                let departure_station: String = Input::new()
                    .with_prompt("新しい出発駅")
                    .interact_text()
                    .unwrap();
                trip.departure_station = departure_station;
            }
            2 => {
                let line: String = Input::new()
                    .with_prompt("新しい路線")
                    .interact_text()
                    .unwrap();
                trip.line = line;
            }
            3 => {
                let train_type: String = Input::new()
                    .with_prompt("新しい列車種別")
                    .interact_text()
                    .unwrap();
                trip.train_type = train_type;
            }
            4 => {
                let destination: String = Input::new()
                    .with_prompt("新しい行先")
                    .interact_text()
                    .unwrap();
                trip.destination = destination;
            }
            5 => {
                let fare: u32 = Input::new()
                    .with_prompt("新しい料金")
                    .interact_text()
                    .unwrap();
                trip.fare = fare;
            }
            6 => {
                let arrival_time: String = Input::new()
                    .with_prompt("新しい到着時刻 (HH:MM)")
                    .interact_text()
                    .unwrap();
                trip.arrival_time = arrival_time;
            }
            7 => {
                let arrival_station: String = Input::new()
                    .with_prompt("新しい到着駅")
                    .interact_text()
                    .unwrap();
                trip.arrival_station = arrival_station;
            }
            8 => {
                if Confirm::new()
                    .with_prompt("本当に削除しますか？")
                    .interact()
                    .unwrap()
                {
                    let mut trips = load_trips();
                    trips.remove(trip_index);
                    save_trips(&trips);
                    println!("移動計画を削除しました。");
                    return;
                }
            }
            9 => break, // 終了
            _ => (),
        }

        // 編集後に計画を保存
        let mut trips = load_trips();
        trips[trip_index] = trip.clone();
        save_trips(&trips);
        println!("変更を保存しました。");
    }
}

fn list_trips() {
    let trips = load_trips();
    if trips.is_empty() {
        println!("保存された旅行計画はありません。");
    } else {
        let total_fare: u32 = trips.iter().map(|trip| trip.fare).sum(); // 合計金額を計算

        // 計画の表示
        for (i, trip) in trips.iter().enumerate() {
            println!(
                "{}. {} {} → {} {} ({}線 {} {}行 ¥{})",
                i + 1,
                trip.departure_time,
                trip.departure_station,
                trip.arrival_time,
                trip.arrival_station,
                trip.line,
                trip.train_type,
                trip.destination,
                trip.fare
            );
        }

        // 合計金額の表示
        println!("\n合計金額: ¥{}", total_fare);
    }
}

fn save_trips(trips: &Vec<Trip>) {
    let json = serde_json::to_string_pretty(trips).unwrap();
    fs::write(FILE_PATH, json).unwrap();
}

fn load_trips() -> Vec<Trip> {
    let data = fs::read_to_string(FILE_PATH).unwrap_or("[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}
