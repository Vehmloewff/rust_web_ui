use rust_web_ui::*;

pub struct Example1;

pub struct Example1Props {}

impl Default for Example1Props {
	fn default() -> Example1Props {
		Example1Props { }
	}
}

impl Widget<'_> for Example1 {
	type Props = Example1Props;

	fn render(mut ctx: Ctx<'_>, props: Example1Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(48)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-4xl")]);
				
				props.child("1", Dynamic).run("ul", |mut props| {
					props.set_attribute("role", "list");
					props.styles(&[Style::Noop("divide-y"), Style::Noop("divide-gray-100")]);
					
					props.child("1", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Leslie Alexander"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("leslie.alexander@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Co-Founder / CEO"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Last seen"));
								props.child("1", Dynamic).run("time", |mut props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.text("3h ago"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Michael Foster"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("michael.foster@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Co-Founder / CTO"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Last seen"));
								props.child("1", Dynamic).run("time", |mut props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.text("3h ago"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Dries Vincent"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("dries.vincent@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Business Relations"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-1.5")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Noop("bg-emerald-500/20"), Style::Padding(Size::Exact(4))]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Width(Size::Exact(6)), Style::Width(Size::Exact(6)), Style::Noop("rounded-full"), Style::Noop("bg-emerald-500")]);
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Online"));
								});
							});
						});
					});
					props.child("7", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Lindsay Walton"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("lindsay.walton@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Front-end Developer"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Last seen"));
								props.child("1", Dynamic).run("time", |mut props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.text("3h ago"));
								});
							});
						});
					});
					props.child("9", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Courtney Henry"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("courtney.henry@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Designer"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Last seen"));
								props.child("1", Dynamic).run("time", |mut props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.text("3h ago"));
								});
							});
						});
					});
					props.child("11", Dynamic).run("li", |mut props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, Style::Noop("gap-x-6"), Style::PaddingY(Size::Exact(20))]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("min-w-0"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Tom Cook"));
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("truncate"), Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("tom.cook@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::Noop("shrink-0"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-col")]), Style::OnScreen(Screen::Small, &[Style::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Director of Product"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-1.5")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Noop("bg-emerald-500/20"), Style::Padding(Size::Exact(4))]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Width(Size::Exact(6)), Style::Width(Size::Exact(6)), Style::Noop("rounded-full"), Style::Noop("bg-emerald-500")]);
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-xs"), Style::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.text("Online"));
								});
							});
						});
					});
				});
			});
		});
	}
}
