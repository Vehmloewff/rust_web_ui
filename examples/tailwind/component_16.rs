use rust_web_ui::*;

pub struct Example16;

pub struct Example16Props {}

impl Default for Example16Props {
	fn default() -> Example16Props {
		Example16Props { }
	}
}

impl Widget<'_> for Example16 {
	type Props = Example16Props;

	fn render(mut ctx: Ctx<'_>, props: Example16Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Style::PaddingX(16), Style::PaddingY(64), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingY(96)]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("h2", |props| {
					props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
					
					props.child("0", Label).run(|props| props.set_text("Customers also purchased"));
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[Style::MarginTop(24), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-10"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("grid-cols-4")]), Screen::ExtraLarge(1, &[NoStyle::Noop("gap-x-8")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), Style::Color(Color::Fg(22)), Screen::Large(&[NoStyle::Noop("aspect-none")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Large(&[Style::Width(320)])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-01.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in black.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), Screen::Large(&[NoStyle::Noop("h-full")]), Screen::Large(&[NoStyle::Noop("w-full")])]);
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(16), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Black"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("$35"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), Style::Color(Color::Fg(22)), Screen::Large(&[NoStyle::Noop("aspect-none")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Large(&[Style::Width(320)])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-02.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in white.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), Screen::Large(&[NoStyle::Noop("h-full")]), Screen::Large(&[NoStyle::Noop("w-full")])]);
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(16), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Aspen White"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("$35"));
							});
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), Style::Color(Color::Fg(22)), Screen::Large(&[NoStyle::Noop("aspect-none")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Large(&[Style::Width(320)])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-03.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in dark gray.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), Screen::Large(&[NoStyle::Noop("h-full")]), Screen::Large(&[NoStyle::Noop("w-full")])]);
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(16), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Charcoal"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("$35"));
							});
						});
					});
					props.child("7", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-md"), Style::Color(Color::Fg(22)), Screen::Large(&[NoStyle::Noop("aspect-none")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75")), Screen::Large(&[Style::Width(320)])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-04.jpg");
								props.set_attribute("alt", "Front of men's Artwork Tee in peach with white and brown dots forming an isometric cube.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), Screen::Large(&[NoStyle::Noop("h-full")]), Screen::Large(&[NoStyle::Noop("w-full")])]);
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(16), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Artwork Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Iso Dots"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("$35"));
							});
						});
					});
				});
			});
		});
	}
}
