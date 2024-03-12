use rust_web_ui::*;

pub struct Example19;

pub struct Example19Props {}

impl Default for Example19Props {
	fn default() -> Example19Props {
		Example19Props { }
	}
}

impl Widget<'_> for Example19 {
	type Props = Example19Props;

	fn render(mut ctx: Ctx<'_>, props: Example19Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("grid"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("grid-cols-1"), Style::ItemsCenter, NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), Style::PaddingX(16), Style::PaddingY(96), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Technical Specifications"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(16), Style::TextColor(Color::Fg(56))]);
						
						props.child("0", Label).run(|props| props.set_text("The walnut wood card tray is precision milled to perfectly fit a stack of Focus cards. The powder coated steel divider separates active cards from new ones, or can be used to archive important task lists."));
					});
					props.child("5", Dynamic).run("dl", |props| {
						props.styles(&[Style::MarginTop(64), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-10"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Small(&[NoStyle::Noop("gap-y-16")]), Screen::Large(&[NoStyle::Noop("gap-x-8")])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Origin"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Designed by Good Goods, Inc."));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Material"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Solid walnut base with rare earth magnets and powder coated steel card cover"));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Dimensions"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text(r#"6.25" x 3.55" x 1.15""#));
							});
						});
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Finish"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Hand sanded and finished with natural oil"));
							});
						});
						props.child("9", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Includes"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Wood card tray and 3 refill packs"));
							});
						});
						props.child("11", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(16)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Considerations"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Made from natural materials. Grain and color vary with each item."));
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("grid-rows-2"), NoStyle::Noop("gap-4"), Screen::Small(&[NoStyle::Noop("gap-6")]), Screen::Large(&[NoStyle::Noop("gap-8")])]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-01.jpg");
						props.set_attribute("alt", "Walnut card tray with white powder coated steel divider and 3 punchout holes.");
						props.styles(&[NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("3", Dynamic).run("img", |props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-02.jpg");
						props.set_attribute("alt", "Top down view of walnut card tray with embedded magnets and card groove.");
						props.styles(&[NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("5", Dynamic).run("img", |props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-03.jpg");
						props.set_attribute("alt", "Side of walnut card tray with card groove and recessed card area.");
						props.styles(&[NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("7", Dynamic).run("img", |props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-04.jpg");
						props.set_attribute("alt", "Walnut card tray filled with cards and card angled in dedicated groove.");
						props.styles(&[NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
				});
			});
		});
	}
}
