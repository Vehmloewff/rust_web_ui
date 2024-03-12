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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Screen::Large(&[Style::MarginX(0)])]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("From the blog"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.set_text("Learn how to grow your business with our expert advice."));
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(40), NoStyle::Noop("grid"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingTop(40), Screen::Small(&[Style::MarginTop(64)]), Screen::Small(&[Style::PaddingTop(64)]), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-3")])]);
					
					props.child("1", Dynamic).run("article", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("max-w-xl"), NoStyle::Noop("flex-col"), NoStyle::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4"), NoStyle::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |props| {
								props.set_attribute("datetime", "2020-03-16");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Mar 16, 2020"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(12), Style::PaddingY(6), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.set_text("Marketing"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(12), NoStyle::Noop("text-lg"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(67)))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("Boost your conversion rate"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(20), NoStyle::Noop("line-clamp-3"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta."));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(32), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Michael Foster"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("Co-Founder / CTO"));
								});
							});
						});
					});
					props.child("3", Dynamic).run("article", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("max-w-xl"), NoStyle::Noop("flex-col"), NoStyle::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4"), NoStyle::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |props| {
								props.set_attribute("datetime", "2020-03-10");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Mar 10, 2020"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(12), Style::PaddingY(6), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.set_text("Sales"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(12), NoStyle::Noop("text-lg"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(67)))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("How to use search engine optimization to drive sales"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(20), NoStyle::Noop("line-clamp-3"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Optio cum necessitatibus dolor voluptatum provident commodi et. Qui aperiam fugiat nemo cumque."));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(32), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Lindsay Walton"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("Front-end Developer"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("article", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("max-w-xl"), NoStyle::Noop("flex-col"), NoStyle::Noop("items-start"), Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4"), NoStyle::Noop("text-xs")]);
							
							props.child("1", Dynamic).run("time", |props| {
								props.set_attribute("datetime", "2020-02-12");
								props.styles(&[Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.set_text("Feb 12, 2020"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6)), Style::PaddingX(12), Style::PaddingY(6), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
								
								props.child("0", Label).run(|props| props.set_text("Business"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative")]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[Style::MarginTop(12), NoStyle::Noop("text-lg"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(67)))]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
									});
									props.child("2", Label).run(|props| props.set_text("Improve your customer experience"));
								});
							});
							props.child("3", Dynamic).run("p", |props| {
								props.styles(&[Style::MarginTop(20), NoStyle::Noop("line-clamp-3"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Cupiditate maiores ullam eveniet adipisci in doloribus nulla minus. Voluptas iusto libero adipisci rem et corporis. Nostrud sint anim sunt aliqua. Nulla eu labore irure incididunt velit cillum quis magna dolore."));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(32), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
								props.set_attribute("alt", "");
								props.styles(&[Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(6))]);
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6")]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
										});
										props.child("2", Label).run(|props| props.set_text("Tom Cook"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::TextColor(Color::Fg(67))]);
									
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
