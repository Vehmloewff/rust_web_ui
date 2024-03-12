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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(64)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("h2", |mut props| {
					props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
					
					props.child("0", Label).run(|props| props.text("Customers also purchased"));
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-6"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-4")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("gap-x-8")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-none")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Exact(320))])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-01.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in black.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)])]);
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Black"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("$35"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-none")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Exact(320))])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-02.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in white.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)])]);
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Aspen White"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("$35"));
							});
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-none")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Exact(320))])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-03.jpg");
								props.set_attribute("alt", "Front of men's Basic Tee in dark gray.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)])]);
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Basic Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Charcoal"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("$35"));
							});
						});
					});
					props.child("7", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-md"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-none")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Exact(320))])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-01-related-product-04.jpg");
								props.set_attribute("alt", "Front of men's Artwork Tee in peach with white and brown dots forming an isometric cube.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)])]);
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.set_attribute("aria-hidden", "true");
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Artwork Tee"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Iso Dots"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("$35"));
							});
						});
					});
				});
			});
		});
	}
}
