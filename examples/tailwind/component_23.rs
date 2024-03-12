use rust_web_ui::*;

pub struct Example23;

pub struct Example23Props {}

impl Default for Example23Props {
	fn default() -> Example23Props {
		Example23Props { }
	}
}

impl Widget<'_> for Example23 {
	type Props = Example23Props;

	fn render(mut ctx: Ctx<'_>, props: Example23Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(64)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("h2", |mut props| {
					props.styles(&[Style::Noop("sr-only")]);
					
					props.child("0", Label).run(|props| props.text("Products"));
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-6"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("grid-cols-4")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("gap-x-8")])]);
					
					props.child("1", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-01.jpg");
								props.set_attribute("alt", "Tall slender porcelain bottle with natural clay textured body and cork stopper.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Earthen Bottle"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$48"));
						});
					});
					props.child("3", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-02.jpg");
								props.set_attribute("alt", "Olive drab green insulated bottle with flared screw lid and flat top.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Nomad Tumbler"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$35"));
						});
					});
					props.child("5", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg");
								props.set_attribute("alt", "Person using a pen to cross a task off a productivity paper card.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Focus Paper Refill"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$89"));
						});
					});
					props.child("7", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-04.jpg");
								props.set_attribute("alt", "Hand holding black machined steel mechanical pencil with brass tip and top.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Machined Mechanical Pencil"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$35"));
						});
					});
					props.child("9", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-05.jpg");
								props.set_attribute("alt", "Paper card sitting upright in walnut card holder on desk.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Focus Card Tray"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$64"));
						});
					});
					props.child("11", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-06.jpg");
								props.set_attribute("alt", "Stack of 3 small drab green cardboard paper card refill boxes with white text.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Focus Multi-Pack"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$39"));
						});
					});
					props.child("13", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-07.jpg");
								props.set_attribute("alt", "Brass scissors with geometric design, black steel finger holes, and included upright brass stand.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Brass Scissors"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$50"));
						});
					});
					props.child("15", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-h-8")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-08.jpg");
								props.set_attribute("alt", "Textured gray felt pouch for paper cards with snap button flap and elastic pen holder loop.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center"), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
							});
						});
						props.child("3", Dynamic).run("h3", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.text("Focus Carry Pouch"));
						});
						props.child("5", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-lg"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$32"));
						});
					});
				});
			});
		});
	}
}
