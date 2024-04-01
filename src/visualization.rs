use charming::{
    component::{Legend, RadarCoordinate, Title},
    element::ItemStyle,
    series::{Radar},
    Chart, ImageRenderer
};

fn main() {
    
    let chart =Chart::new()
        .title(Title::new().text("Basic Radar Chart"))
        .legend(Legend::new().data(vec!["Allocated Budget", "Actual Spending"]))
        .radar(RadarCoordinate::new().indicator(vec![
            ("College", 0, 1000),
            ("Software Development", 0, 1000),
            ("Exercize", 0, 1000),
            ("Eating", 0, 1000),
            ("Sleeping", 0, 1000),
            ("Video Games", 0, 1000),
        ]))
        .series(Radar::new().name("Budget vs spending").data(vec![
            (
                vec![100, 800, 140, 60, 250, 70],
                "Allocated Budget",
            ),
        ]));

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "nightingal.svg");
}