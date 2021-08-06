use crate::*;
use plotters::prelude::*;

pub fn draw(
    data: [i32; NUMBER_OF_YEARS_USIZE],
    file_name: String,
    plot_title: String,
    analysis_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: String = format!("output/{}/{}", analysis_type, String::clone(&file_name));
    let mut biggest_element = *data.iter().max_by_key(|p| *p).unwrap();

    let margin_up = biggest_element as f64;
    let margin_up: i32 = (margin_up / 20.0) as i32;

    biggest_element += margin_up;

    let end_year = START_YEAR + (data.len() as i32) - 1;

    let root_area = BitMapBackend::new(&file_name, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(plot_title, ("sans-serif", 40))
        .build_cartesian_2d((START_YEAR..end_year).into_segmented(), 0..biggest_element)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series((1..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x + START_YEAR - 1);
        let x1 = SegmentValue::Exact(x + START_YEAR);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();

    Ok(())
}
