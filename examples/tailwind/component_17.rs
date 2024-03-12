use rust_web_ui::*;

pub struct Example17;

pub struct Example17Props {}

impl Default for Example17Props {
	fn default() -> Example17Props {
		Example17Props { }
	}
}

impl Widget<'_> for Example17 {
	type Props = Example17Props;

	fn render(mut ctx: Ctx<'_>, props: Example17Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Style::PaddingY(64), Screen::Small(&[Style::PaddingY(96)]), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[Style::PaddingY(128)])]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.set_text("Collections"));
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[Style::MarginTop(24), Style::SpaceY(48), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("grid-cols-3")]), Screen::Large(&[NoStyle::Noop("gap-x-6")]), Screen::Large(&[Style::SpaceY(0)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("relative"), Style::Width(320), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-white"), Screen::Small(&[NoStyle::Noop("aspect-h-1")]), Screen::Small(&[NoStyle::Noop("aspect-w-2")]), Screen::Large(&[NoStyle::Noop("aspect-h-1")]), Screen::Large(&[NoStyle::Noop("aspect-w-1")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Small(&[Style::Width(256)])]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-01.jpg");
									props.set_attribute("alt", "Desk with leather desk pad, walnut desk organizer, wireless keyboard and mouse, and porcelain mug.");
									props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("Desk and Office"));
								});
							});
							props.child("5", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Work from home accessories"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("relative"), Style::Width(320), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-white"), Screen::Small(&[NoStyle::Noop("aspect-h-1")]), Screen::Small(&[NoStyle::Noop("aspect-w-2")]), Screen::Large(&[NoStyle::Noop("aspect-h-1")]), Screen::Large(&[NoStyle::Noop("aspect-w-1")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Small(&[Style::Width(256)])]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-02.jpg");
									props.set_attribute("alt", "Wood table with porcelain mug, leather journal, brass pen, leather key ring, and a houseplant.");
									props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("Self-Improvement"));
								});
							});
							props.child("5", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Journals and note-taking"));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("relative"), Style::Width(320), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-white"), Screen::Small(&[NoStyle::Noop("aspect-h-1")]), Screen::Small(&[NoStyle::Noop("aspect-w-2")]), Screen::Large(&[NoStyle::Noop("aspect-h-1")]), Screen::Large(&[NoStyle::Noop("aspect-w-1")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Small(&[Style::Width(256)])]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-03.jpg");
									props.set_attribute("alt", "Collection of four insulated travel bottles on wooden shelf.");
									props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("Travel"));
								});
							});
							props.child("5", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Daily commute essentials"));
							});
						});
					});
				});
			});
		});
	}
}
