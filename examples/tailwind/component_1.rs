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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingY(48), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-4xl")]);
				
				props.child("1", Dynamic).run("ul", |props| {
					props.set_attribute("role", "list");
					props.styles(&[NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-100")]);
					
					props.child("1", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Leslie Alexander"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("leslie.alexander@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Co-Founder / CEO"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Last seen"));
								props.child("1", Dynamic).run("time", |props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.set_text("3h ago"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Michael Foster"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("michael.foster@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Co-Founder / CTO"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Last seen"));
								props.child("1", Dynamic).run("time", |props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.set_text("3h ago"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Dries Vincent"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("dries.vincent@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Business Relations"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(4), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-1.5")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), NoStyle::Noop("bg-emerald-500/20"), Style::Padding(4)]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::Width(6), Style::Width(6), NoStyle::Noop("rounded-full"), NoStyle::Noop("bg-emerald-500")]);
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Online"));
								});
							});
						});
					});
					props.child("7", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Lindsay Walton"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("lindsay.walton@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Front-end Developer"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Last seen"));
								props.child("1", Dynamic).run("time", |props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.set_text("3h ago"));
								});
							});
						});
					});
					props.child("9", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Courtney Henry"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("courtney.henry@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Designer"));
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(4), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Last seen"));
								props.child("1", Dynamic).run("time", |props| {
									props.set_attribute("datetime", "2023-01-23T13:23Z");
									
									props.child("0", Label).run(|props| props.set_text("3h ago"));
								});
							});
						});
					});
					props.child("11", Dynamic).run("li", |props| {
						props.styles(&[Style::Flex, Style::JustifyBetween, NoStyle::Noop("gap-x-6"), Style::PaddingY(20)]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("min-w-0"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(48), Style::Width(48), NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Tom Cook"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(4), NoStyle::Noop("truncate"), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("tom.cook@example.com"));
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("hidden"), NoStyle::Noop("shrink-0"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-col")]), Screen::Small(&[NoStyle::Noop("items-end")])]);
							
							props.child("1", Dynamic).run("p", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Director of Product"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(4), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-1.5")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), NoStyle::Noop("bg-emerald-500/20"), Style::Padding(4)]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::Width(6), Style::Width(6), NoStyle::Noop("rounded-full"), NoStyle::Noop("bg-emerald-500")]);
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(56))]);
									
									props.child("0", Label).run(|props| props.set_text("Online"));
								});
							});
						});
					});
				});
			});
		});
	}
}
