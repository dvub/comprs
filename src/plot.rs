use circular_buffer::CircularBuffer;
use plotters::prelude::*;
use rand::Rng;

use crate::dsp::Compressor;

const OUT_FILE_NAME: &str = "plots/0.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let data: Vec<f32> = (0..1000).map(|_| rng.gen_range(0.4..=0.5)).collect();

    let comp = Compressor {
        rms: 0.0,
        envelope: 0.0,
        gain: 1.0,
        squared_sum: 0.0,
        buf: CircularBuffer::<44, f32>::new(),
    };

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Area Chart Demo", ("sans-serif", 40))
        .build_cartesian_2d(0.0..(data.len() as f32 - 1.0), 0.0..1.0f32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            data.iter().enumerate().map(|(x, y)| (x as f32, *y)),
            0.0,
            RED.mix(0.2),
        )
        .border_style(RED),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
