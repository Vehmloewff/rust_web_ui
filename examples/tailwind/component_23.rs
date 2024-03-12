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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Style::PaddingX(16), Style::PaddingY(64), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingY(96)]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("h2", |props| {
					props.styles(&[NoStyle::Noop("sr-only")]);
					
					props.child("0", Label).run(|props| props.set_text("Products"));
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-10"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("grid-cols-3")]), Screen::ExtraLarge(1, &[NoStyle::Noop("grid-cols-4")]), Screen::ExtraLarge(1, &[NoStyle::Noop("gap-x-8")])]);
					
					props.child("1", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-01.jpg");
								props.set_attribute("alt", "Tall slender porcelain bottle with natural clay textured body and cork stopper.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Earthen Bottle"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$48"));
						});
					});
					props.child("3", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-02.jpg");
								props.set_attribute("alt", "Olive drab green insulated bottle with flared screw lid and flat top.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Nomad Tumbler"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$35"));
						});
					});
					props.child("5", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg");
								props.set_attribute("alt", "Person using a pen to cross a task off a productivity paper card.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Focus Paper Refill"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$89"));
						});
					});
					props.child("7", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-04.jpg");
								props.set_attribute("alt", "Hand holding black machined steel mechanical pencil with brass tip and top.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Machined Mechanical Pencil"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$35"));
						});
					});
					props.child("9", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-05.jpg");
								props.set_attribute("alt", "Paper card sitting upright in walnut card holder on desk.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Focus Card Tray"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$64"));
						});
					});
					props.child("11", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-06.jpg");
								props.set_attribute("alt", "Stack of 3 small drab green cardboard paper card refill boxes with white text.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Focus Multi-Pack"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$39"));
						});
					});
					props.child("13", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-07.jpg");
								props.set_attribute("alt", "Brass scissors with geometric design, black steel finger holes, and included upright brass stand.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Brass Scissors"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$50"));
						});
					});
					props.child("15", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("group")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(22)), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-h-8")]), Screen::ExtraLarge(1, &[NoStyle::Noop("aspect-w-7")])]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-08.jpg");
								props.set_attribute("alt", "Textured gray felt pouch for paper cards with snap button flap and elastic pen holder loop.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center"), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
							});
						});
						props.child("3", Dynamic).run("h3", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
							
							props.child("0", Label).run(|props| props.set_text("Focus Carry Pouch"));
						});
						props.child("5", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$32"));
						});
					});
				});
			});
		});
	}
}
