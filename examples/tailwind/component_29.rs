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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("grid"), Style::Noop("max-w-7xl"), Style::Noop("gap-x-8"), Style::Noop("gap-y-20"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("grid-cols-3")])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("max-w-2xl")]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.text("Meet our leadership"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.text("Libero fames augue nisl porttitor nisi, quis. Id ac elit odio vitae elementum enim vitae ullamcorper suspendisse."));
					});
				});
				props.child("3", Dynamic).run("ul", |mut props| {
					props.set_attribute("role", "list");
					props.styles(&[Style::Noop("grid"), Style::Noop("gap-x-8"), Style::Noop("gap-y-12"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-y-16")]), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("col-span-2")])]);
					
					props.child("1", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Leslie Alexander"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Co-Founder / CEO"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Michael Foster"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Co-Founder / CTO"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Dries Vincent"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Business Relations"));
								});
							});
						});
					});
					props.child("7", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Lindsay Walton"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Front-end Developer"));
								});
							});
						});
					});
					props.child("9", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Courtney Henry"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Designer"));
								});
							});
						});
					});
					props.child("11", Dynamic).run("li", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(64)), Style::Width(Size::Exact(64)), Style::Noop("rounded-full")]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Tom Cook"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Director of Product"));
								});
							});
						});
					});
				});
			});
		});
	}
}
