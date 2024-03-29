fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut reports: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        let report: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        // println!("report {:?}", report);
        if report.len() > 0 {
            reports.push(report);
        }
    }

    reports
}

fn generate_difference(prediction: &Vec<i64>) -> Vec<i64> {
    let mut new_prediction: Vec<i64> = Vec::new();

    let new_prediciton_len = (prediction.len() - 1) as usize;

    for i in 0..new_prediciton_len {
        let diff = prediction[i + 1] - prediction[i];
        new_prediction.push(diff)
    }

    new_prediction
}

fn day_1(report: Vec<i64>) -> i64 {
    let mut predictions: Vec<Vec<i64>> = Vec::new();

    predictions.push(report);

    while !predictions.last().unwrap().iter().all(|&a| a == 0) {
        let prediction = generate_difference(predictions.last().unwrap());

        predictions.push(prediction);
    }

    for i in (0..predictions.len()).rev() {
        // set bottom as zero
        let is_bottom: bool = i == predictions.len() - 1;
        let mut extrapolation: i64 = 0;
        if !is_bottom {
            let prev_row_last_num = predictions.get(i + 1).unwrap().last().unwrap();
            let curr_row_last_num = predictions.get(i).unwrap().last().unwrap();
            extrapolation = prev_row_last_num + curr_row_last_num;
        }

        let prediction: &mut Vec<i64> = predictions.get_mut(i).unwrap();
        prediction.push(extrapolation);
    }

    // for prediction in &predictions {
    //     println!("prediction {:?}", prediction);
    // }
    // println!("");

    predictions.get(0).unwrap().last().unwrap().to_owned()
}

fn main() {
    let input = include_str!("../input.txt");

    let reports = parse_input(input);

    // println!("reports {:?}", reports);

    let mut day_1_results: Vec<i64> = Vec::new();
    for report in reports {
        let day_1_result = day_1(report);
        day_1_results.push(day_1_result);
    }

    println!("day_1_results {:?}", day_1_results);
    println!(
        "day_1_result {:?}",
        day_1_results.into_iter().reduce(|a, b| a + b).unwrap()
    );
}
