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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("grid"), Style::Noop("max-w-2xl"), Style::Noop("grid-cols-1"), Style::ItemsCenter, Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.text("Technical Specifications"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(16)), Style::TextColor(Color::Fg(56))]);
						
						props.child("0", Label).run(|props| props.text("The walnut wood card tray is precision milled to perfectly fit a stack of Focus cards. The powder coated steel divider separates active cards from new ones, or can be used to archive important task lists."));
					});
					props.child("5", Dynamic).run("dl", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(64)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-6"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-y-16")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Origin"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Designed by Good Goods, Inc."));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Material"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Solid walnut base with rare earth magnets and powder coated steel card cover"));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Dimensions"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text(r#"6.25" x 3.55" x 1.15""#));
							});
						});
						props.child("7", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Finish"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Hand sanded and finished with natural oil"));
							});
						});
						props.child("9", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Includes"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Wood card tray and 3 refill packs"));
							});
						});
						props.child("11", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(16))]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Considerations"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Made from natural materials. Grain and color vary with each item."));
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("grid-rows-2"), Style::Noop("gap-4"), Style::OnScreen(Screen::Small, &[Style::Noop("gap-6")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-8")])]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-01.jpg");
						props.set_attribute("alt", "Walnut card tray with white powder coated steel divider and 3 punchout holes.");
						props.styles(&[Style::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-02.jpg");
						props.set_attribute("alt", "Top down view of walnut card tray with embedded magnets and card groove.");
						props.styles(&[Style::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("5", Dynamic).run("img", |mut props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-03.jpg");
						props.set_attribute("alt", "Side of walnut card tray with card groove and recessed card area.");
						props.styles(&[Style::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
					props.child("7", Dynamic).run("img", |mut props| {
						props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-feature-03-detail-04.jpg");
						props.set_attribute("alt", "Walnut card tray filled with cards and card angled in dedicated groove.");
						props.styles(&[Style::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
					});
				});
			});
		});
	}
}
