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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::PaddingY(Size::Exact(64)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::PaddingY(Size::Exact(128))])]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.text("Collections"));
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(48)), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-6")]), Style::OnScreen(Screen::Large, &[Style::SpaceY(Size::Exact(0))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("relative"), Style::Width(Size::Exact(320)), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Noop("bg-white"), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-w-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-w-1")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Exact(256))])]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-01.jpg");
									props.set_attribute("alt", "Desk with leather desk pad, walnut desk organizer, wireless keyboard and mouse, and porcelain mug.");
									props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("Desk and Office"));
								});
							});
							props.child("5", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Work from home accessories"));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("relative"), Style::Width(Size::Exact(320)), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Noop("bg-white"), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-w-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-w-1")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Exact(256))])]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-02.jpg");
									props.set_attribute("alt", "Wood table with porcelain mug, leather journal, brass pen, leather key ring, and a houseplant.");
									props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("Self-Improvement"));
								});
							});
							props.child("5", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Journals and note-taking"));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("relative"), Style::Width(Size::Exact(320)), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Noop("bg-white"), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Small, &[Style::Noop("aspect-w-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-h-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-w-1")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Exact(256))])]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-02-edition-03.jpg");
									props.set_attribute("alt", "Collection of four insulated travel bottles on wooden shelf.");
									props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
								});
							});
							props.child("3", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("Travel"));
								});
							});
							props.child("5", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Daily commute essentials"));
							});
						});
					});
				});
			});
		});
	}
}
