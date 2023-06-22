use std::ops::Range;
use plotters::prelude::*;

pub struct Plot {
    x_size: u32,
    y_size: u32,
    x_range: Range<i32>,
    y_range: Range<f64>,
}

impl Plot {
    pub fn new(size_x: u32, size_y: u32, range_x: Range<i32>, range_y: Range<f64>) -> Self {
        let plot = Plot{x_size: size_x, y_size: size_y, x_range: range_x, y_range: range_y};
        return plot;
    }


    //plots a timeseries stored as a Vec<(i32, f64)>
    pub fn plot_multiple_timeseries(&self, path: &str, caption: &str, data: Vec<Vec<(i32, f64)>>, color_start: &RGBColor, color_end: &RGBColor, labels: Vec<String>) {

        //initialize image
        let backend = SVGBackend::new(path , (self.x_size, self.y_size)).into_drawing_area();
        backend.fill(&WHITE).unwrap();

        //initialize context of image
        let mut chartbuilder = ChartBuilder::on(&backend);
        chartbuilder.caption(caption, ("Arial", 30))
        .set_all_label_area_size(40);

        //draw cartesian plane
        let mut chartcontext = chartbuilder.build_cartesian_2d(
            self.x_range.clone(), 
            self.y_range.clone()
        ).unwrap();

        chartcontext.configure_mesh().draw().unwrap();

        //plot timeseries
        let gradient: Vec<RGBColor> = self.get_gradient(color_start, color_end, data.len());
        for plot_id in 0..data.len() {
            let grad = gradient[plot_id].clone();
            chartcontext
                .draw_series(LineSeries::new(data[plot_id].clone(), gradient[plot_id]))
                .unwrap()
                .label(labels[plot_id].to_string())
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], grad));
        }

        //draw labels
        chartcontext
        .configure_series_labels()
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperLeft)
        .draw().unwrap();
    }

    pub fn plot_timeseries(&self, path: &str, caption: &str, data: Vec<(i32, f64)>, color: &RGBColor, label: &str) {

        //initialize image
        let backend = SVGBackend::new(path , (self.x_size, self.y_size)).into_drawing_area();
        backend.fill(&WHITE).unwrap();

        //initialize context of image
        let mut chartbuilder = ChartBuilder::on(&backend);
        chartbuilder.caption(caption, ("Arial", 30))
        .set_all_label_area_size(40);

        //draw cartesian plane
        let mut chartcontext = chartbuilder.build_cartesian_2d(
            self.x_range.clone(), 
            self.y_range.clone()
        ).unwrap();

        chartcontext.configure_mesh().draw().unwrap();

        //plot timeseries

        chartcontext.draw_series(LineSeries::new(
            data,
            &color,
        )).unwrap()
        .label(label.to_string())
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

        //draw labels
        chartcontext
        .configure_series_labels()
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperLeft)
        .draw().unwrap();
    }

    //generates a gradient between two colors in rgb space
    pub fn get_gradient(&self, start_color: &RGBColor, end_color: &RGBColor, steps: usize) -> Vec<RGBColor> {
        let (r1, g1, b1) = start_color.rgb();
        let (r2, g2, b2) = end_color.rgb();
    
        let r_step = ((r2 as f64 - r1 as f64) / (steps - 1) as f64) as i16;
        let g_step = ((g2 as f64 - g1 as f64) / (steps - 1) as f64) as i16;
        let b_step = ((b2 as f64 - b1 as f64) / (steps - 1) as f64) as i16;
    
        let mut gradient = Vec::with_capacity(steps);
        for i in 0..steps {
            let r = r1 as i16 + i as i16 * r_step;
            let g = g1 as i16 + i as i16 * g_step;
            let b = b1 as i16 + i as i16 * b_step;
            gradient.push(RGBColor(r as u8, g as u8, b as u8));
        }
    
        return gradient
    }

}