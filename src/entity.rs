use cursive::align::HAlign;
use cursive::CursiveRunnable;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SliderView, TextView};

pub fn create_player(siv: &mut CursiveRunnable) {
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Character Name"))
                .child(EditView::new().with_name("character_name"))
                .child(DummyView.fixed_height(1))
                .child(TextView::new("Strength"))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("0").with_name("strength_amount"))
                        .child(DummyView.fixed_width(1))
                        .child(
                            SliderView::horizontal(11)
                                .on_change(|s, v| {
                                    s.call_on_name("strength_amount", |view: &mut TextView| {
                                        view.set_content(v.to_string());
                                    });
                                })
                        )
                )
                .child(TextView::new("Stamina"))
                .child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::new("0").with_name("stamina_amount")
                        )
                        .child(DummyView.fixed_width(1))
                        .child(
                            SliderView::horizontal(11)
                                .on_change(|s, v| {
                                    s.call_on_name("stamina_amount", |view: &mut TextView| {
                                        view.set_content(v.to_string());
                                    });
                                })
                        )
                )
                .fixed_width(30),
        )
            .title("Create Character")
            .button("Create", |s| {
                let character_name = s.call_on_name("character_name", |view: &mut EditView| {
                    view.get_content().to_string()
                });

                let stamina = s.call_on_name("stamina_amount", |view: &mut SliderView| {
                    view.get_value()
                });

                //test(&character_name.unwrap(), &stamina.unwrap());
            })
            .h_align(HAlign::Left),
    );
}