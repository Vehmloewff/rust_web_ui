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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-none")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("overflow-hidden"), Style::Noop("bg-white"), Style::Noop("shadow"), Style::OnScreen(Screen::Small, &[Style::Noop("rounded-lg")])]);
						
						props.child("1", Dynamic).run("ul", |mut props| {
							props.set_attribute("role", "list");
							props.styles(&[Style::Noop("divide-y"), Style::Noop("divide-gray-200"), Style::Noop("opacity-25")]);
							
							props.child("1", Dynamic).run("li", |mut props| {
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("truncate"), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.text("Back End Developer"));
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(8)), Style::Flex, Style::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Success(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::OnScreen(Screen::Small, &[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::MarginRight(Size::Exact(24)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.text("Engineering"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.text("Remote"));
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("li", |mut props| {
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("truncate"), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.text("Front End Developer"));
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(8)), Style::Flex, Style::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Success(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::OnScreen(Screen::Small, &[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::MarginRight(Size::Exact(24)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.text("Engineering"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.text("Remote"));
											});
										});
									});
								});
							});
							props.child("5", Dynamic).run("li", |mut props| {
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("truncate"), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600")]);
												
												props.child("0", Label).run(|props| props.text("User Interface Designer"));
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(8)), Style::Flex, Style::Noop("flex-shrink-0")]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-full"), Style::Color(Color::Success(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Success(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-green-600/20")]);
													
													props.child("0", Label).run(|props| props.text("Full-time"));
												});
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::JustifyBetween]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::OnScreen(Screen::Small, &[Style::Flex])]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::MarginRight(Size::Exact(24)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("2", Label).run(|props| props.text("Design"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
												});
												props.child("2", Label).run(|props| props.text("Remote"));
											});
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(12)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::JustifyBetween, Style::OnScreen(Screen::Small, &[Style::Noop("hidden")])]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-300"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("0", Label).run(|props| props.text("Previous"));
								});
								props.child("3", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Noop("relative"), Style::MarginLeft(Size::Exact(12)), Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-300"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("0", Label).run(|props| props.text("Next"));
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::ItemsCenter]), Style::OnScreen(Screen::Small, &[Style::JustifyBetween])]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.child("1", Dynamic).run("p", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										
										props.child("0", Label).run(|props| props.text("Showing"));
										//  space 
										props.child("3", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.text("1"));
										});
										//  space 
										props.child("6", Label).run(|props| props.text("to"));
										//  space 
										props.child("9", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.text("10"));
										});
										//  space 
										props.child("12", Label).run(|props| props.text("of"));
										//  space 
										props.child("15", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("font-medium")]);
											
											props.child("0", Label).run(|props| props.text("97"));
										});
										//  space 
										props.child("18", Label).run(|props| props.text("results"));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.child("1", Dynamic).run("nav", |mut props| {
										props.styles(&[Style::Noop("isolate"), Style::InlineFlex, Style::Noop("-space-x-px"), Style::Noop("rounded-md"), Style::Noop("shadow-sm")]);
										props.set_attribute("aria-label", "Pagination");
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-l-md"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("Previous"));
											});
											props.child("3", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
											});
										});
										//  Current: "z-10 bg-indigo-600 text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600", Default: "text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:outline-offset-0" 
										props.child("5", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.set_attribute("aria-current", "page");
											props.styles(&[Style::Noop("relative"), Style::Noop("z-10"), Style::InlineFlex, Style::ItemsCenter, Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::OnFocus(&[Style::Noop("z-20")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
											
											props.child("0", Label).run(|props| props.text("1"));
										});
										props.child("7", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.text("2"));
										});
										props.child("9", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::Noop("hidden"), Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")]), Style::OnScreen(Screen::Medium, &[Style::InlineFlex])]);
											
											props.child("0", Label).run(|props| props.text("3"));
										});
										props.child("11", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.text("..."));
										});
										props.child("13", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::Noop("hidden"), Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")]), Style::OnScreen(Screen::Medium, &[Style::InlineFlex])]);
											
											props.child("0", Label).run(|props| props.text("8"));
										});
										props.child("15", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.text("9"));
										});
										props.child("17", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("0", Label).run(|props| props.text("10"));
										});
										props.child("19", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-r-md"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("z-20")]), Style::OnFocus(&[Style::Noop("outline-offset-0")])]);
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("Next"));
											});
											props.child("3", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
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
