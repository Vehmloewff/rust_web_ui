use rust_web_ui::*;

pub struct Example2;

pub struct Example2Props {}

impl Default for Example2Props {
	fn default() -> Example2Props {
		Example2Props { }
	}
}

impl Widget<'_> for Example2 {
	type Props = Example2Props;

	fn render(mut ctx: Ctx<'_>, props: Example2Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingY(24), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-none")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow"), Screen::Small(&[NoStyle::Noop("rounded-lg")])]);
						
						props.child("1", Dynamic).run("ul", |props| {
							props.set_attribute("role", "list");
							props.styles(&[NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-200"), NoStyle::Noop("opacity-25")]);
							
							props.child("1", Dynamic).run("li", |props| {
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingX(16), Style::PaddingY(16), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("truncate"), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.set_text("Back End Developer"));
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::MarginLeft(8), Style::Flex, NoStyle::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Success(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.set_text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Screen::Small(&[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginRight(24), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.set_text("Engineering"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.set_text("Remote"));
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("li", |props| {
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingX(16), Style::PaddingY(16), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("truncate"), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.set_text("Front End Developer"));
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::MarginLeft(8), Style::Flex, NoStyle::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Success(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.set_text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Screen::Small(&[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginRight(24), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.set_text("Engineering"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.set_text("Remote"));
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("li", |props| {
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingX(16), Style::PaddingY(16), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("truncate"), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.set_text("User Interface Designer"));
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::MarginLeft(8), Style::Flex, NoStyle::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Success(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.set_text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Screen::Small(&[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[Style::MarginRight(24), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.set_text("Design"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.set_text("Remote"));
											});
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingY(12), Screen::Small(&[Style::PaddingX(24)])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), Style::JustifyBetween, Screen::Small(&[NoStyle::Noop("hidden")])]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-300"), NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("0", Label).run(|props| props.set_text("Previous"));
								});
								props.child("3", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[NoStyle::Noop("relative"), Style::MarginLeft(12), Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-gray-300"), NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("0", Label).run(|props| props.set_text("Next"));
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("hidden"), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::ItemsCenter]), Screen::Small(&[Style::JustifyBetween])]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.child("1", Dynamic).run("p", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										
										props.child("0", Label).run(|props| props.set_text("Showing"));
										//  space 
										props.child("3", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.set_text("1"));
										});
										//  space 
										props.child("6", Label).run(|props| props.set_text("to"));
										//  space 
										props.child("9", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.set_text("10"));
										});
										//  space 
										props.child("12", Label).run(|props| props.set_text("of"));
										//  space 
										props.child("15", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.set_text("97"));
										});
										//  space 
										props.child("18", Label).run(|props| props.set_text("results"));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.child("1", Dynamic).run("nav", |props| {
										props.styles(&[NoStyle::Noop("isolate"), Style::InlineFlex, NoStyle::Noop("-space-x-px"), NoStyle::Noop("rounded-md"), NoStyle::Noop("shadow-sm")]);
										props.set_attribute("aria-label", "Pagination");
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-l-md"), Style::PaddingX(8), Style::PaddingY(8), Style::TextColor(Color::Fg(44)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("Previous"));
											});
											props.child("3", Icon).run(|props| {
												props.style(&[Style::Width(20), Style::Width(20)]);
											});
										});
										//  Current: "z-10 bg-indigo-600 text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600", Default: "text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:outline-offset-0" 
										props.child("5", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.set_attribute("aria-current", "page");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10"), Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("bg-indigo-600"), Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), Action::Hover(&[NoStyle::Noop("z-20")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
											
											props.child("0", Label).run(|props| props.set_text("1"));
										});
										props.child("7", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.set_text("2"));
										});
										props.child("9", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("hidden"), Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")]), Screen::Medium(&[Style::InlineFlex])]);
											
											props.child("0", Label).run(|props| props.set_text("3"));
										});
										props.child("11", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.set_text("..."));
										});
										props.child("13", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("hidden"), Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")]), Screen::Medium(&[Style::InlineFlex])]);
											
											props.child("0", Label).run(|props| props.set_text("8"));
										});
										props.child("15", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.set_text("9"));
										});
										props.child("17", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.set_text("10"));
										});
										props.child("19", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-r-md"), Style::PaddingX(8), Style::PaddingY(8), Style::TextColor(Color::Fg(44)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("z-20")]), Action::Hover(&[NoStyle::Noop("outline-offset-0")])]);
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("Next"));
											});
											props.child("3", Icon).run(|props| {
												props.style(&[Style::Width(20), Style::Width(20)]);
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
