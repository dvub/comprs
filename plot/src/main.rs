/*
use dsp::{Compressor, LevelDetectionType};
use nih_plug::util::db_to_gain;
use plotters::prelude::*;
// TODO: make this a timestamp or something idk
const OUT_FILE_NAME: &str = "plots/0.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let len = 44_100;
    //let mut rng = rand::thread_rng();
    let mut data: Vec<f32> = vec![0.0; len];
    for (index, value) in data.iter_mut().enumerate() {
        let q = len / 4;
        let factor = {
            if index >= (q * 3) {
                -12.0
            } else if index >= (q * 2) {
                0.0
            } else if index >= (q) {
                -9.0
            } else {
                -12.0
            }
        };
        *value = (index as f32 * 0.1).sin() * db_to_gain(factor);
    }
    let threshold = -10.0;
    let ratio = 100.0;
    let knee = 5.0;
    let attack_time = 0.005;
    let release_time = 0.05;
    let mut comp = Compressor::new(
        attack_time,
        release_time,
        threshold,
        ratio,
        knee,
        LevelDetectionType::Rms,
    );
    let compressed_data: Vec<((f32, f32), f32)> = data
        .iter()
        .map(|sample| {
            let result = comp.process(*sample);
            (result, comp.get_average_gain())
        })
        .collect();
    let (compression_results, envelopes): ((Vec<f32>, Vec<f32>), Vec<f32>) =
        compressed_data.into_iter().unzip();

    let root = BitMapBackend::new(OUT_FILE_NAME, (2000, 2000)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Compressor Gain", ("JetBrains Mono", 40))
        .build_cartesian_2d(0.0..(data.len() as f32 - 1.0), -1.0..1.0f32)?;

    chart.configure_mesh().disable_x_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(x, y)| (x as f32, *y)),
        RED,
    ))?;

    chart.draw_series(LineSeries::new(
        compression_results
            .0
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y)),
        GREEN,
    ))?;
    chart.draw_series(LineSeries::new(
        envelopes.iter().enumerate().map(|(x, y)| (x as f32, *y)),
        BLUE,
    ))?;
    chart.draw_series(LineSeries::new(
        vec![db_to_gain(threshold); data.len()]
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y)),
        BLACK,
    ))?;

    /*
    chart.draw_series(LineSeries::new(
        compression_results
            .1
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y)),
        BLACK,
    ))?;



    chart.draw_series(LineSeries::new(
        vec![-threshold; 44100]
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y)),
        BLACK,
    ))?;
    */
    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure the proper output dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
*/
fn main() {}
