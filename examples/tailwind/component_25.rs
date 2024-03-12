use rust_web_ui::*;

pub struct Example25;

pub struct Example25Props {}

impl Default for Example25Props {
	fn default() -> Example25Props {
		Example25Props { }
	}
}

impl Widget<'_> for Example25 {
	type Props = Example25Props;

	fn render(mut ctx: Ctx<'_>, props: Example25Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("relative"), Style::Noop("overflow-hidden"), Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::PaddingBottom(Size::Exact(320)), Style::PaddingTop(Size::Exact(64)), Style::OnScreen(Screen::Small, &[Style::PaddingBottom(Size::Exact(160))]), Style::OnScreen(Screen::Small, &[Style::PaddingTop(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::PaddingBottom(Size::Exact(192))]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(160))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("relative"), Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::Noop("static")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("max-w-lg")])]);
						
						props.child("1", Dynamic).run("h1", |mut props| {
							props.styles(&[Style::Noop("text-4xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-6xl")])]);
							
							props.child("0", Label).run(|props| props.text("Summer styles are finally here"));
						});
						props.child("3", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-xl"), Style::TextColor(Color::Fg(56))]);
							
							props.child("0", Label).run(|props| props.text("This year, our new summer collection will shelter you from the harsh elements of a world that doesn't care if you live or die."));
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40))]);
							
							//  Decorative image grid 
							props.child("3", Dynamic).run("div", |mut props| {
								props.set_attribute("aria-hidden", "true");
								props.styles(&[Style::Noop("pointer-events-none"), Style::OnScreen(Screen::Large, &[Style::Noop("absolute")]), Style::OnScreen(Screen::Large, &[Style::Noop("inset-y-0")]), Style::OnScreen(Screen::Large, &[Style::Noop("mx-auto")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")])]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("transform"), Style::OnScreen(Screen::Small, &[Style::Noop("left-1/2")]), Style::OnScreen(Screen::Small, &[Style::Noop("top-0")]), Style::OnScreen(Screen::Small, &[Style::Noop("translate-x-8")]), Style::OnScreen(Screen::Large, &[Style::Noop("left-1/2")]), Style::OnScreen(Screen::Large, &[Style::Noop("top-1/2")]), Style::OnScreen(Screen::Large, &[Style::Noop("-translate-y-1/2")]), Style::OnScreen(Screen::Large, &[Style::Noop("translate-x-8")])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::SpaceX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::SpaceX(Size::Exact(32))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("grid"), Style::Noop("flex-shrink-0"), Style::Noop("grid-cols-1"), Style::Noop("gap-y-6"), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::OnScreen(Screen::Small, &[Style::Noop("opacity-0")]), Style::OnScreen(Screen::Large, &[Style::Noop("opacity-100")])]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-01.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-02.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("grid"), Style::Noop("flex-shrink-0"), Style::Noop("grid-cols-1"), Style::Noop("gap-y-6"), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-03.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-04.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("5", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-05.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
										});
										props.child("5", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("grid"), Style::Noop("flex-shrink-0"), Style::Noop("grid-cols-1"), Style::Noop("gap-y-6"), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-06.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Width(Size::Exact(256)), Style::Width(Size::Exact(176)), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-07.jpg");
													props.set_attribute("alt", "");
													props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::InlineBlock, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-transparent"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(32)), Style::PaddingY(Size::Exact(12)), Style::Noop("text-center"), Style::Noop("font-medium"), Style::Noop("text-white"), Style::OnHover(&[Style::Noop("bg-indigo-700")])]);
								
								props.child("0", Label).run(|props| props.text("Shop Collection"));
							});
						});
					});
				});
			});
		});
	}
}
