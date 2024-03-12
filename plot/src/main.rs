use comprs::dsp::Compressor;
use plotters::prelude::*;
use rand::Rng;
// use crate::dsp::Compressor;

const OUT_FILE_NAME: &str = "plots/0.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let data: Vec<f32> = (0..44_100).map(|_| rng.gen_range(-1.1..=1.1f32)).collect();

    let mut comp = Compressor::default();

    let threshold = 0.7;
    let slope = 0.5;

    // let window_width = 1.0 * 1e-3;
    let attack_time = 0.1 * 1e-3;
    let release_time = 300.0 * 1e-3;
    let compressed_data: Vec<(f32, f32)> = data
        .iter()
        .map(|sample| {
            let result = comp.process(*sample, attack_time, release_time, threshold, slope);

            (result, comp.average_gain)
        })
        .collect();
    let (values, envelopes): (Vec<f32>, Vec<f32>) = compressed_data.into_iter().unzip();

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Area Chart Demo", ("sans-serif", 40))
        .build_cartesian_2d(0.0..(data.len() as f32 - 1.0), -1.0..1.0f32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(x, y)| (x as f32, *y)),
        RED.mix(0.2),
    ))?;

    chart.draw_series(LineSeries::new(
        values.iter().enumerate().map(|(x, y)| (x as f32, *y)),
        GREEN.mix(0.2),
    ))?;

    chart.draw_series(LineSeries::new(
        envelopes.iter().enumerate().map(|(x, y)| (x as f32, *y)),
        BLUE,
    ))?;

    /*
    chart.draw_series(
        AreaSeries::new(
            gains.iter().enumerate().map(|(x, y)| (x as f32, *y)),
            0.0,
            BLUE.mix(0.2),
        )
        .border_style(BLUE),
    )?;
    */

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure the proper output dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
