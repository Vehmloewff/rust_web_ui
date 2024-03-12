use rust_web_ui::*;

pub struct Example21;

pub struct Example21Props {}

impl Default for Example21Props {
	fn default() -> Example21Props {
		Example21Props { }
	}
}

impl Widget<'_> for Example21 {
	type Props = Example21Props;

	fn render(mut ctx: Ctx<'_>, props: Example21Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				
				//  Mobile filter dialog 
				props.child("3", Dynamic).run("div", |props| {
					props.set_attribute("x-show", "open");
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-40"), Screen::Large(&[NoStyle::Noop("hidden")])]);
					props.set_attribute("x-description", "Off-canvas filters for mobile, show/hide based on off-canvas filters state.");
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:enter-start", "opacity-0");
						props.set_attribute("x-transition:enter-end", "opacity-100");
						props.set_attribute("x-transition:leave", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:leave-start", "opacity-100");
						props.set_attribute("x-transition:leave-end", "opacity-0");
						props.set_attribute("x-description", "Off-canvas menu backdrop, show/hide based on off-canvas menu state.");
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("bg-black"), NoStyle::Noop("bg-opacity-25")]);
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-40"), Style::Flex]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:enter-start", "translate-x-full");
							props.set_attribute("x-transition:enter-end", "translate-x-0");
							props.set_attribute("x-transition:leave", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:leave-start", "translate-x-0");
							props.set_attribute("x-transition:leave-end", "translate-x-full");
							props.set_attribute("x-description", "Off-canvas menu, show/hide based on off-canvas menu state.");
							props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("ml-auto"), Style::Flex, NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("max-w-xs"), NoStyle::Noop("flex-col"), NoStyle::Noop("overflow-y-auto"), NoStyle::Noop("bg-white"), Style::PaddingY(16), Style::PaddingBottom(48), NoStyle::Noop("shadow-xl")]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingX(16)]);
								
								props.child("1", Dynamic).run("h2", |props| {
									props.styles(&[NoStyle::Noop("text-lg"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Filters"));
								});
								props.child("3", Dynamic).run("button", |props| {
									props.set_attribute("type", "button");
									props.styles(&[NoStyle::Noop("-mr-2"), Style::Flex, Style::Width(40), Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::Padding(8), Style::TextColor(Color::Fg(44))]);
									props.set_attribute("@click", "open = false");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text("Close menu"));
									});
									props.child("3", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24)]);
									});
								});
							});
							//  Filters 
							props.child("5", Dynamic).run("form", |props| {
								props.styles(&[Style::MarginTop(16), NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200")]);
								
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Categories"));
								});
								props.child("3", Dynamic).run("ul", |props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::PaddingX(8), Style::PaddingY(12), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(8), Style::PaddingY(12)]);
											
											props.child("0", Label).run(|props| props.set_text("Totes"));
										});
									});
									props.child("3", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(8), Style::PaddingY(12)]);
											
											props.child("0", Label).run(|props| props.set_text("Backpacks"));
										});
									});
									props.child("5", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(8), Style::PaddingY(12)]);
											
											props.child("0", Label).run(|props| props.set_text("Travel Bags"));
										});
									});
									props.child("7", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(8), Style::PaddingY(12)]);
											
											props.child("0", Label).run(|props| props.set_text("Hip Bags"));
										});
									});
									props.child("9", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Block, Style::PaddingX(8), Style::PaddingY(12)]);
											
											props.child("0", Label).run(|props| props.set_text("Laptop Sleeves"));
										});
									});
								});
								props.child("5", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-mx-2"), NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingX(8), Style::PaddingY(12), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-0");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Color"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-0");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-0");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "white");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("White"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-1");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "beige");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Beige"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-2");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "blue");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Blue"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-3");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "brown");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Brown"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-4");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "green");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Green"));
												});
											});
											props.child("11", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-color-5");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "purple");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-color-5");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Purple"));
												});
											});
										});
									});
								});
								props.child("7", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-mx-2"), NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingX(8), Style::PaddingY(12), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Category"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-1");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-category-0");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "new-arrivals");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-category-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("New Arrivals"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-category-1");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "sale");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-category-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Sale"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-category-2");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "travel");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-category-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Travel"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-category-3");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "organization");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-category-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Organization"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-category-4");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "accessories");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-category-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Accessories"));
												});
											});
										});
									});
								});
								props.child("9", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-mx-2"), NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingX(8), Style::PaddingY(12), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-mobile-2");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Size"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-mobile-2");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-0");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "2l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("2L"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-1");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "6l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("6L"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-2");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "12l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("12L"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-3");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "18l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("18L"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-4");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "20l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("20L"));
												});
											});
											props.child("11", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-mobile-size-5");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "40l");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-mobile-size-5");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1"), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("40L"));
												});
											});
										});
									});
								});
							});
						});
					});
				});
				props.child("5", Dynamic).run("main", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("items-baseline"), Style::JustifyBetween, NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200"), Style::PaddingBottom(24), Style::PaddingTop(96)]);
						
						props.child("1", Dynamic).run("h1", |props| {
							props.styles(&[NoStyle::Noop("text-4xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("New Arrivals"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.set_attribute("x-data", "Components.menu({ open: false })");
								props.set_attribute("x-init", "init()");
								props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
								props.set_attribute("@click.away", "onClickAway($event)");
								props.styles(&[NoStyle::Noop("relative"), Style::InlineBlock, NoStyle::Noop("text-left")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.child("1", Dynamic).run("button", |props| {
										props.set_attribute("type", "button");
										props.styles(&[NoStyle::Noop("group"), Style::InlineFlex, Style::JustifyCenter, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(100))])]);
										props.set_attribute("id", "menu-button");
										props.set_attribute("x-ref", "button");
										props.set_attribute("@click", "onButtonClick()");
										props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
										props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
										props.set_attribute("aria-expanded", "false");
										props.set_attribute("aria-haspopup", "true");
										props.set_attribute("x-bind:aria-expanded", "open.toString()");
										props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
										props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
										
										props.child("0", Label).run(|props| props.set_text("Sort"));
										props.child("1", Icon).run(|props| {
											props.style(&[NoStyle::Noop("-mr-1"), Style::MarginLeft(4), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(56)))]);
										});
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "transition ease-out duration-100");
									props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
									props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave", "transition ease-in duration-75");
									props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("right-0"), NoStyle::Noop("z-10"), Style::MarginTop(8), Style::Width(160), NoStyle::Noop("origin-top-right"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-2xl"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-black"), NoStyle::Noop("ring-opacity-5"), Action::Hover(&[NoStyle::Noop("outline-none")])]);
									props.set_attribute("x-ref", "menu-items");
									props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
									props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
									props.set_attribute("role", "menu");
									props.set_attribute("aria-orientation", "vertical");
									props.set_attribute("aria-labelledby", "menu-button");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
									props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
									props.set_attribute("@keydown.tab", "open = false");
									props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
									props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingY(4)]);
										props.set_attribute("role", "none");
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
											props.set_attribute("x-state:on", "Active");
											props.set_attribute("x-state:off", "Not Active");
											props.set_attribute("x-state:on:option.current", "Selected");
											props.set_attribute("x-state:off:option.current", "Not Selected");
											props.set_attribute("x-state-description", r#"Selected: "font-medium text-gray-900", Not Selected: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 0 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-0");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 0)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.set_text("Most Popular"));
										});
										props.child("3", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-1");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 1)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.set_text("Best Rating"));
										});
										props.child("5", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 2 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-2");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 2)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.set_text("Newest"));
										});
										props.child("7", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 3 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-3");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 3)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.set_text("Price: Low to High"));
										});
										props.child("9", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::TextColor(Color::Fg(56)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
											props.set_attribute("x-state-description", r#"undefined: "font-medium text-gray-900", undefined: "text-gray-500""#);
											props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 4 }");
											props.set_attribute("role", "menuitem");
											props.set_attribute("tabindex", "-1");
											props.set_attribute("id", "menu-item-4");
											props.set_attribute("@mouseenter", "onMouseEnter($event)");
											props.set_attribute("@mousemove", "onMouseMove($event, 4)");
											props.set_attribute("@mouseleave", "onMouseLeave($event)");
											props.set_attribute("@click", "open = false; focusButton()");
											
											props.child("0", Label).run(|props| props.set_text("Price: High to Low"));
										});
									});
								});
							});
							props.child("3", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("-m-2"), Style::MarginLeft(20), Style::Padding(8), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))]), Screen::Small(&[Style::MarginLeft(28)])]);
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("View grid"));
								});
								props.child("3", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(20)]);
								});
							});
							props.child("5", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("-m-2"), Style::MarginLeft(16), Style::Padding(8), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))]), Screen::Small(&[Style::MarginLeft(24)]), Screen::Large(&[NoStyle::Noop("hidden")])]);
								props.set_attribute("@click", "open = true");
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Filters"));
								});
								props.child("3", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(20)]);
								});
							});
						});
					});
					props.child("3", Dynamic).run("section", |props| {
						props.set_attribute("aria-labelledby", "products-heading");
						props.styles(&[Style::PaddingBottom(96), Style::PaddingTop(24)]);
						
						props.child("1", Dynamic).run("h2", |props| {
							props.set_attribute("id", "products-heading");
							props.styles(&[NoStyle::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.set_text("Products"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Screen::Large(&[NoStyle::Noop("grid-cols-4")])]);
							
							//  Filters 
							props.child("3", Dynamic).run("form", |props| {
								props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::Block])]);
								
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Categories"));
								});
								props.child("3", Dynamic).run("ul", |props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::SpaceY(16), NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200"), Style::PaddingBottom(24), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("1", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.set_text("Totes"));
										});
									});
									props.child("3", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.set_text("Backpacks"));
										});
									});
									props.child("5", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.set_text("Travel Bags"));
										});
									});
									props.child("7", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.set_text("Hip Bags"));
										});
									});
									props.child("9", Dynamic).run("li", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											
											props.child("0", Label).run(|props| props.set_text("Laptop Sleeves"));
										});
									});
								});
								props.child("5", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200"), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingY(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-0");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Color"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-0");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(16)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-0");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "white");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("White"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-1");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "beige");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Beige"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-2");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "blue");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Blue"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-3");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "brown");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Brown"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-4");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "green");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Green"));
												});
											});
											props.child("11", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-color-5");
													props.set_attribute("name", "color[]");
													props.set_attribute("value", "purple");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-color-5");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Purple"));
												});
											});
										});
									});
								});
								props.child("7", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200"), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingY(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Category"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-1");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(16)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-category-0");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "new-arrivals");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-category-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("New Arrivals"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-category-1");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "sale");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-category-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Sale"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-category-2");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "travel");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-category-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Travel"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-category-3");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "organization");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-category-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Organization"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-category-4");
													props.set_attribute("name", "category[]");
													props.set_attribute("value", "accessories");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-category-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("Accessories"));
												});
											});
										});
									});
								});
								props.child("9", Dynamic).run("div", |props| {
									props.set_attribute("x-data", "{ open: false }");
									props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200"), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("-my-3"), NoStyle::Noop("flow-root")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.set_attribute("x-description", "Expand/collapse section button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("bg-white"), Style::PaddingY(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											props.set_attribute("aria-controls", "filter-section-2");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Size"));
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(24), Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(20), Style::Width(20)]);
												});
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingTop(24)]);
										props.set_attribute("x-description", "Filter section, show/hide based on section state.");
										props.set_attribute("id", "filter-section-2");
										props.set_attribute("x-show", "open");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::SpaceY(16)]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-0");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "2l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-0");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("2L"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-1");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "6l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-1");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("6L"));
												});
											});
											props.child("5", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-2");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "12l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-2");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("12L"));
												});
											});
											props.child("7", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-3");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "18l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-3");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("18L"));
												});
											});
											props.child("9", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-4");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "20l");
													props.set_attribute("type", "checkbox");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-4");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("20L"));
												});
											});
											props.child("11", Dynamic).run("div", |props| {
												props.styles(&[Style::Flex, Style::ItemsCenter]);
												
												props.child("1", Dynamic).run("input", |props| {
													props.set_attribute("id", "filter-size-5");
													props.set_attribute("name", "size[]");
													props.set_attribute("value", "40l");
													props.set_attribute("type", "checkbox");
													props.set_attribute("checked", "");
													props.styles(&[Style::Width(16), Style::Width(16), Style::Rounded, NoStyle::Noop("border-gray-300"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("ring-indigo-500")])]);
												});
												props.child("3", Dynamic).run("label", |props| {
													props.set_attribute("for", "filter-size-5");
													props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
													
													props.child("0", Label).run(|props| props.set_text("40L"));
												});
											});
										});
									});
								});
							});
							//  Product grid 
							props.child("7", Dynamic).run("div", |props| {
								props.styles(&[Screen::Large(&[NoStyle::Noop("col-span-3")])]);
								
								props.child("1", Dynamic).run("x-placeholder", |props| {
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("relative"), Style::Width(384), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-xl"), NoStyle::Noop("border"), NoStyle::Noop("border-dashed"), NoStyle::Noop("border-gray-400"), NoStyle::Noop("opacity-75"), Screen::Large(&[NoStyle::Noop("h-full")])]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("stroke-gray-900/10")]);
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
