use rust_web_ui::*;

pub struct Example26;

pub struct Example26Props {}

impl Default for Example26Props {
	fn default() -> Example26Props {
		Example26Props { }
	}
}

impl Widget<'_> for Example26 {
	type Props = Example26Props;

	fn render(mut ctx: Ctx<'_>, props: Example26Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))])]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.text("From the blog"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.text("Learn how to grow your business with our expert advice."));
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(40)), Style::Noop("grid"), Style::Noop("max-w-2xl"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingTop(Size::Exact(40)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(64))]), Style::OnScreen(Screen::Small, &[Style::PaddingTop(Size::Exact(64))]), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")])]);
					
					props.child("1", Dynamic).run("article", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("max-w-xl"), Style::Noop("flex-col"), Style::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4"), Style::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |mut props| {
								props.set_attribute("datetime", "2020-03-16");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Mar 16, 2020"));
							});
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("relative"), Style::Noop("z-10"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(6)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.text("Marketing"));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(12)), Style::Noop("text-lg"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(67))])]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("Boost your conversion rate"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(20)), Style::Noop("line-clamp-3"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta."));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(32)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Michael Foster"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("Co-Founder / CTO"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("article", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("max-w-xl"), Style::Noop("flex-col"), Style::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4"), Style::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |mut props| {
								props.set_attribute("datetime", "2020-03-10");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Mar 10, 2020"));
							});
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("relative"), Style::Noop("z-10"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(6)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.text("Sales"));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(12)), Style::Noop("text-lg"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(67))])]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("How to use search engine optimization to drive sales"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(20)), Style::Noop("line-clamp-3"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Optio cum necessitatibus dolor voluptatum provident commodi et. Qui aperiam fugiat nemo cumque."));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(32)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Lindsay Walton"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text("Front-end Developer"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("article", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("max-w-xl"), Style::Noop("flex-col"), Style::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4"), Style::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |mut props| {
								props.set_attribute("datetime", "2020-02-12");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Feb 12, 2020"));
							});
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("relative"), Style::Noop("z-10"), Style::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(6)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.text("Business"));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(12)), Style::Noop("text-lg"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(67))])]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.text("Improve your customer experience"));
								});
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(20)), Style::Noop("line-clamp-3"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.text("Cupiditate maiores ullam eveniet adipisci in doloribus nulla minus. Voluptas iusto libero adipisci rem et corporis. Nostrud sint anim sunt aliqua. Nulla eu labore irure incididunt velit cillum quis magna dolore."));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(32)), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.text("Tom Cook"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
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
