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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[Style::PaddingBottom(320), Style::PaddingTop(64), Screen::Small(&[Style::PaddingBottom(160)]), Screen::Small(&[Style::PaddingTop(96)]), Screen::Large(&[Style::PaddingBottom(192)]), Screen::Large(&[Style::PaddingTop(160)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(16), Screen::Small(&[NoStyle::Noop("static")]), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Screen::Small(&[NoStyle::Noop("max-w-lg")])]);
						
						props.child("1", Dynamic).run("h1", |props| {
							props.styles(&[NoStyle::Noop("text-4xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-6xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("Summer styles are finally here"));
						});
						props.child("3", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-xl"), Style::TextColor(Color::Fg(56))]);
							
							props.child("0", Label).run(|props| props.set_text("This year, our new summer collection will shelter you from the harsh elements of a world that doesn't care if you live or die."));
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(40)]);
							
							//  Decorative image grid 
							props.child("3", Dynamic).run("div", |props| {
								props.set_attribute("aria-hidden", "true");
								props.styles(&[NoStyle::Noop("pointer-events-none"), Screen::Large(&[NoStyle::Noop("absolute")]), Screen::Large(&[NoStyle::Noop("inset-y-0")]), Screen::Large(&[NoStyle::Noop("mx-auto")]), Screen::Large(&[NoStyle::Noop("w-full")]), Screen::Large(&[NoStyle::Noop("max-w-7xl")])]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("transform"), Screen::Small(&[NoStyle::Noop("left-1/2")]), Screen::Small(&[NoStyle::Noop("top-0")]), Screen::Small(&[NoStyle::Noop("translate-x-8")]), Screen::Large(&[NoStyle::Noop("left-1/2")]), Screen::Large(&[NoStyle::Noop("top-1/2")]), Screen::Large(&[NoStyle::Noop("-translate-y-1/2")]), Screen::Large(&[NoStyle::Noop("translate-x-8")])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::SpaceX(24), Screen::Large(&[Style::SpaceX(32)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-y-6"), Screen::Large(&[NoStyle::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Screen::Small(&[NoStyle::Noop("opacity-0")]), Screen::Large(&[NoStyle::Noop("opacity-100")])]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-01.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-02.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-y-6"), Screen::Large(&[NoStyle::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-03.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-04.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-05.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
										});
										props.child("5", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-y-6"), Screen::Large(&[NoStyle::Noop("gap-y-8")])]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-06.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Width(256), Style::Width(176), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/home-page-03-hero-image-tile-07.jpg");
													props.set_attribute("alt", "");
													props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::InlineBlock, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-transparent"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(32), Style::PaddingY(12), NoStyle::Noop("text-center"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-white"), Action::Hover(&[NoStyle::Noop("bg-indigo-700")])]);
								
								props.child("0", Label).run(|props| props.set_text("Shop Collection"));
							});
						});
					});
				});
			});
		});
	}
}
