use rust_web_ui::*;

pub struct Example29;

pub struct Example29Props {}

impl Default for Example29Props {
	fn default() -> Example29Props {
		Example29Props { }
	}
}

impl Widget<'_> for Example29 {
	type Props = Example29Props;

	fn render(mut ctx: Ctx<'_>, props: Example29Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("grid"), NoStyle::Noop("max-w-7xl"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-20"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)]), Screen::ExtraLarge(1, &[NoStyle::Noop("grid-cols-3")])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("max-w-2xl")]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Meet our leadership"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.set_text("Libero fames augue nisl porttitor nisi, quis. Id ac elit odio vitae elementum enim vitae ullamcorper suspendisse."));
					});
				});
				props.child("3", Dynamic).run("ul", |props| {
					props.set_attribute("role", "list");
					props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-12"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Small(&[NoStyle::Noop("gap-y-16")]), Screen::ExtraLarge(1, &[NoStyle::Noop("col-span-2")])]);
					
					props.child("1", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Leslie Alexander"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Co-Founder / CEO"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Michael Foster"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Co-Founder / CTO"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Dries Vincent"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Business Relations"));
								});
							});
						});
					});
					props.child("7", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Lindsay Walton"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Front-end Developer"));
								});
							});
						});
					});
					props.child("9", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Courtney Henry"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Designer"));
								});
							});
						});
					});
					props.child("11", Dynamic).run("li", |props| {
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(64), Style::Width(64), NoStyle::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Tom Cook"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.set_text("Director of Product"));
								});
							});
						});
					});
				});
			});
		});
	}
}
